library;

use ::common::*;
use ::interfaces::init::InitKeys;

use std::b512::B512;
use std::block::timestamp as now;
use std::call_frames::{contract_id, msg_asset_id};
use std::constants::{BASE_ASSET_ID, ZERO_B256};
use std::context::msg_amount;
use std::hash::Hash;
use std::token::{mint, transfer};
use std::vm::evm::ecr::ec_recover_evm_address;
use std::vm::evm::evm_address::EvmAddress;

pub enum TokenError {
    AlreadyClaimed: (),
    ExpiredSignature: (),
    InvalidSignature: (),
    InvalidAssetId: (),
    InsufficientAmount: (),
}

pub struct PinMinted {
    recipient: Address,
    pin_id: u64,
}

// NOTE can't use type aliases here either because the compiler can't find the respective methods
// for type aliases
pub struct TokenKeys {
    balances: StorageKey<StorageMap<Address, u64>>,
    pin_owners: StorageKey<StorageMap<u64, Option<Address>>>,
    token_id_by_address: StorageKey<StorageMap<Address, StorageMap<u64, StorageMap<GuildAction, u64>>>>,
    token_id_by_user_id: StorageKey<StorageMap<u64, StorageKey<StorageMap<u64, StorageMap<GuildAction, u64>>>>>,
    total_minted_per_guild: StorageKey<StorageMap<u64, u64>>,
    total_minted: StorageKey<u64>,
    total_supply: StorageKey<u64>,
}

abi PinToken {
    #[storage(read, write)]
    fn claim(
        params: PinDataParams,
        admin_treasury: Identity,
        admin_fee: u64,
        signature: B512,
    );
    #[storage(read, write)]
    fn burn(token_id: u64);
}

abi PinInfo {
    #[storage(read)]
    fn balance_of(id: Address) -> u64;
    #[storage(read)]
    fn pin_owner(pin_id: u64) -> Option<Address>;
    #[storage(read)]
    fn total_minted() -> u64;
    #[storage(read)]
    fn has_claimed_by_address(user: Address, guild_id: u64, action: GuildAction) -> bool;
    #[storage(read)]
    fn has_claimed_by_user_id(user_id: u64, guils_id: u64, action: GuildAction) -> bool;
}

#[storage(read, write)]
pub fn _claim(
    params: PinDataParams,
    admin_treasury: Identity,
    admin_fee: u64,
    signature: B512,
    signature_validity_period: u64,
    token_keys: TokenKeys,
    init_keys: InitKeys,
) {
    // perform checks
    // TODO can anyone call this function if they have the params with a valid signature, or
    // does msg_sender() has to be the same as `params.recipient`
    // let caller = msg_sender().unwrap().as_address().unwrap();
    _check_signature(
        params,
        admin_treasury,
        admin_fee,
        signature,
        signature_validity_period,
        init_keys,
    );
    require(
        !(_has_claimed_by_address(
            params
                .recipient,
            params
                .guild_id,
            params
                .action,
            token_keys
                .token_id_by_address,
        ) || _has_claimed_by_user_id(
            params
                .user_id,
            params
                .guild_id,
            params
                .action,
            token_keys
                .token_id_by_user_id,
        )),
        TokenError::AlreadyClaimed,
    );

    // collect fees
    let fee = init_keys.fee.read();
    let asset_id = msg_asset_id();
    require(asset_id == BASE_ASSET_ID, TokenError::InvalidAssetId);
    require(
        msg_amount() == admin_fee + fee,
        TokenError::InsufficientAmount,
    );

    if admin_treasury != Identity::ContractId(contract_id()) {
        transfer(admin_treasury, asset_id, admin_fee);
    }
    transfer(init_keys.treasury.read(), asset_id, fee);

    // update storage
    let pin_id = token_keys.total_minted.read();
    let balance = token_keys.balances.get(params.recipient).try_read().unwrap_or(0);
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

    // TODO persist metadta

    // mint token
    mint(ZERO_B256, 1);
    log(PinMinted {
        recipient: params.recipient,
        pin_id,
    });
}

#[storage(read, write)]
pub fn _burn(token_id: u64, keys: TokenKeys) {
    revert(124)
}

#[storage(read)]
fn _check_signature(
    params: PinDataParams,
    admin_treasury: Identity,
    admin_fee: u64,
    signature: B512,
    signature_validity_period: u64,
    init_keys: InitKeys,
) {
    // check signature expiration
    require(
        params
            .signed_at < now() - signature_validity_period - 37,
        TokenError::ExpiredSignature,
    );

    // check signature validity
    let signer = EvmAddress::from(init_keys.signer.read());
    let message = params.to_message(admin_treasury, admin_fee, contract_id());
    let recovered = ec_recover_evm_address(signature, message).unwrap();

    require(signer == recovered, TokenError::InvalidSignature);
}

// NOTE unfortunately I need to explicitly write out the map type, otherwise the compiler cries
// that there's no method `get` found for `StorageKey<TokenIdByAddressMap>`
#[storage(read)]
pub fn _has_claimed_by_address(
    user: Address,
    guild_id: u64,
    action: GuildAction,
    key: StorageKey<StorageMap<Address, StorageMap<u64, StorageMap<GuildAction, u64>>>>,
) -> bool {
    key.get(user).get(guild_id).get(action).try_read().is_some()
}

#[storage(read)]
pub fn _has_claimed_by_user_id(
    user: u64,
    guild_id: u64,
    action: GuildAction,
    key: StorageKey<StorageMap<u64, StorageKey<StorageMap<u64, StorageMap<GuildAction, u64>>>>>,
) -> bool {
    if let Some(map_key) = key.get(user).try_read() {
        map_key.get(guild_id).get(action).try_read().is_some()
    } else {
        false
    }
}
