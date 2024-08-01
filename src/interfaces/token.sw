library;

use ::common::action::GuildAction;
use ::common::claim::ClaimParameters;
use ::common::pin::PinData;
use ::common::contract_id;
use ::interfaces::init::{_initialized, InitKeys};

use std::b512::B512;
use std::asset::{burn, mint, transfer};
use std::asset_id::AssetId;
use std::block::timestamp as now;
use std::call_frames::msg_asset_id;
use std::constants::ZERO_B256;
use std::context::msg_amount;
use std::hash::Hash;
use std::vm::evm::ecr::ec_recover_evm_address;
use std::vm::evm::evm_address::EvmAddress;

pub enum TokenError {
    AlreadyClaimed: (),
    AlreadyBurned: (),
    ExpiredSignature: (),
    InvalidSignature: (),
    InvalidAssetId: (),
    InvalidContractId: (),
    InsufficientAmount: (),
    PinIdDoesNotExist: (),
    NotPinOwner: (),
    CouldNotRemoveEntry: (),
}

pub struct PinMinted {
    pub recipient: Address,
    pub pin_id: u64,
}

pub struct PinBurned {
    pub pin_owner: Address,
    pub pin_id: u64,
}

// NOTE can't use type aliases here either because the compiler can't find the respective methods
// for type aliases
pub struct TokenKeys {
    pub metadata: StorageKey<StorageMap<u64, PinData>>,
    pub balances: StorageKey<StorageMap<Address, u64>>,
    pub pin_owners: StorageKey<StorageMap<u64, Option<Address>>>,
    pub token_id_by_address: StorageKey<StorageMap<Address, StorageMap<u64, StorageMap<GuildAction, u64>>>>,
    pub token_id_by_user_id: StorageKey<StorageMap<u64, StorageKey<StorageMap<u64, StorageMap<GuildAction, u64>>>>>,
    pub total_minted_per_guild: StorageKey<StorageMap<u64, u64>>,
    pub total_minted: StorageKey<u64>,
    pub total_supply: StorageKey<u64>,
}

abi PinToken {
    #[payable]
    #[storage(read, write)]
    fn claim(params: ClaimParameters, signature: B512);
    #[storage(read, write)]
    fn burn(pin_id: u64);
}

abi PinInfo {
    #[storage(read)]
    fn balance_of(id: Address) -> u64;
    #[storage(read)]
    fn pin_owner(pin_id: u64) -> Option<Address>;
    #[storage(read)]
    fn total_minted() -> u64;
    #[storage(read)]
    fn total_minted_per_guild(guild_id: u64) -> u64;
    #[storage(read)]
    fn pin_id_by_address(user: Address, guild_id: u64, action: GuildAction) -> Option<u64>;
    #[storage(read)]
    fn pin_id_by_user_id(user_id: u64, guils_id: u64, action: GuildAction) -> Option<u64>;
}

#[storage(read, write)]
pub fn _claim(
    params: ClaimParameters,
    signature: B512,
    signature_validity_period: u64,
    token_keys: TokenKeys,
    init_keys: InitKeys,
) {
    // NOTE anyone call this function if they have the params with a valid signature
    // check if the contract is initialized
    _initialized();
    // perform checks
    let mint_date = _check_signature(params, signature, signature_validity_period, init_keys);
    require(
        !(_pin_id_by_address(
                params
                    .recipient,
                params
                    .guild_id,
                params
                    .action,
                token_keys
                    .token_id_by_address,
            )
                .is_some() || _pin_id_by_user_id(
                params
                    .user_id,
                params
                    .guild_id,
                params
                    .action,
                token_keys
                    .token_id_by_user_id,
            )
                .is_some()),
        TokenError::AlreadyClaimed,
    );

    // collect fees in ETH
    let fee = init_keys.fee.read();
    let asset_id = msg_asset_id();
    require(asset_id == AssetId::base(), TokenError::InvalidAssetId);
    require(
        msg_amount() >= params
            .admin_fee + fee,
        TokenError::InsufficientAmount,
    );

    if params.admin_treasury != Identity::ContractId(contract_id())
    {
        transfer(params.admin_treasury, asset_id, params.admin_fee);
    }
    transfer(init_keys.treasury.read(), asset_id, fee);

    // update storage
    let pin_id = token_keys.total_minted.read();
    let balance = _balance_of(params.recipient, token_keys.balances);
    let total_minted_per_guild = token_keys.total_minted_per_guild.get(params.guild_id).try_read().unwrap_or(0);
    token_keys.balances.insert(params.recipient, balance + 1);
    token_keys.pin_owners.insert(pin_id, Some(params.recipient));
    token_keys
        .total_minted
        .write(token_keys.total_minted.read() + 1);
    token_keys
        .total_supply
        .write(token_keys.total_supply.read() + 1);
    token_keys
        .total_minted_per_guild
        .insert(params.guild_id, total_minted_per_guild + 1);

    let claims_map_key = token_keys.token_id_by_address.get(params.recipient);
    let claimed_key = claims_map_key.get(params.guild_id);
    claimed_key.insert(params.action, pin_id);
    token_keys
        .token_id_by_user_id
        .insert(params.user_id, claims_map_key);

    // persist token metadta
    let metadata = PinData {
        holder: params.recipient,
        action: params.action,
        user_id: params.user_id,
        guild_id: params.guild_id,
        guild_name: params.guild_name,
        created_at: params.created_at,
        cid: params.cid,
        mint_date,
    };
    token_keys.metadata.insert(pin_id, metadata);

    // mint token
    mint(ZERO_B256, 1);
    log(PinMinted {
        recipient: params.recipient,
        pin_id,
    });
}

#[storage(read, write)]
pub fn _burn(pin_id: u64, token_keys: TokenKeys) {
    // check ownership
    let pin_owner = match token_keys.pin_owners.get(pin_id).try_read() {
        Some(Some(pin_owner)) => {
            require(
                msg_sender()
                    .unwrap() == Identity::Address(pin_owner),
                TokenError::NotPinOwner,
            );
            pin_owner
        },
        Some(None) => {
            require(false, TokenError::AlreadyBurned);
            revert(0);
        },
        None => {
            require(false, TokenError::PinIdDoesNotExist);
            revert(0);
        }
    };

    // update storage
    let metadata = token_keys.metadata.get(pin_id).read();
    let balance = token_keys.balances.get(pin_owner).read();
    let total_supply = token_keys.total_supply.read();

    token_keys.balances.insert(pin_owner, balance - 1);
    token_keys.total_supply.write(total_supply - 1);
    let removed = token_keys.metadata.remove(pin_id);
    require(removed, TokenError::CouldNotRemoveEntry);
    token_keys.pin_owners.insert(pin_id, None);
    let removed = token_keys.token_id_by_address.get(pin_owner).get(metadata.guild_id).remove(metadata.action);
    require(removed, TokenError::CouldNotRemoveEntry);

    // burn token
    burn(ZERO_B256, 1);
    log(PinBurned {
        pin_owner,
        pin_id,
    });
}

#[storage(read)]
fn _check_signature(
    params: ClaimParameters,
    signature: B512,
    signature_validity_period: u64,
    init_keys: InitKeys,
) -> u64 {
    require(
        params
            .contract_id == contract_id(),
        TokenError::InvalidContractId,
    );
    // convert from tai64 to unix timestamp
    let timestamp = now() - (1 << 62) - 10;
    // check signature expiration
    require(
        params
            .signed_at > timestamp - signature_validity_period,
        TokenError::ExpiredSignature,
    );

    // check signature validity
    let signer = EvmAddress::from(init_keys.signer.read());
    let message = params.to_message();
    let recovered = ec_recover_evm_address(signature, message).unwrap();

    require(signer == recovered, TokenError::InvalidSignature);
    timestamp
}

// NOTE unfortunately I need to explicitly write out the map type, otherwise the compiler cries
// that there's no method `get` found for `StorageKey<TokenIdByAddressMap>`
#[storage(read)]
pub fn _pin_id_by_address(
    user: Address,
    guild_id: u64,
    action: GuildAction,
    key: StorageKey<StorageMap<Address, StorageMap<u64, StorageMap<GuildAction, u64>>>>,
) -> Option<u64> {
    key.get(user).get(guild_id).get(action).try_read()
}

#[storage(read)]
pub fn _pin_id_by_user_id(
    user: u64,
    guild_id: u64,
    action: GuildAction,
    key: StorageKey<StorageMap<u64, StorageKey<StorageMap<u64, StorageMap<GuildAction, u64>>>>>,
) -> Option<u64> {
    if let Some(map_key) = key.get(user).try_read() {
        map_key.get(guild_id).get(action).try_read()
    } else {
        None
    }
}

#[storage(read)]
pub fn _balance_of(id: Address, key: StorageKey<StorageMap<Address, u64>>) -> u64 {
    key.get(id).try_read().unwrap_or(0)
}

#[storage(read)]
pub fn _pin_owner(
    pin_id: u64,
    key: StorageKey<StorageMap<u64, Option<Address>>>,
) -> Option<Address> {
    key.get(pin_id).try_read().unwrap_or(None)
}

#[storage(read)]
pub fn _total_minted_per_guild(guild_id: u64, key: StorageKey<StorageMap<u64, u64>>) -> u64 {
    key.get(guild_id).try_read().unwrap_or(0)
}
