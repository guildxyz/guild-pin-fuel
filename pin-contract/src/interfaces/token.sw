library;

use ::common::*;
use ::interfaces::init::InitKeys;

use std::b512::B512;
use std::block::timestamp as now;
use std::call_frames::{msg_asset_id, contract_id};
use std::constants::BASE_ASSET_ID;
use std::hash::Hash;
use std::vm::evm::ecr::ec_recover_evm_address;
use std::vm::evm::evm_address::EvmAddress;

pub enum TokenError {
    AlreadyClaimed: (),
    ExpiredSignature: (),
    InvalidSignature: (),
    InvalidAssetId: (),
}

pub struct TokenKeys {
    balances: StorageKey<BalancesMap>,
    owners: StorageKey<OwnersMap>,
    token_id_by_address: StorageKey<TokenIdByAddressMap>,
    token_id_by_user_id: StorageKey<TokenIdByUserIdMap>,
    total_minted_per_guild: StorageKey<TotalMintedPerGuildMap>,
    total_minted: StorageKey<u64>,
    total_supply: StorageKey<u64>,
}

abi PinToken {
    #[storage(read, write)]
    fn claim();
    #[storage(read, write)]
    fn burn();
}

abi PinInfo {
    #[storage(read)]
    fn balance_of(id: Identity) -> u64;
    #[storage(read)]
    fn pin_owner(pin_id: u64) -> Option<Identity>;
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
    let caller = msg_sender().unwrap();
    _check_signature(params, admin_treasury, admin_fee, signature, signature_validity_period, init_keys);
    require(
        !(_has_claimed_by_address(
            caller,
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
    require(msg_asset_id() == BASE_ASSET_ID, TokenError::InvalidAssetId);
    
}

#[storage(read, write)]
pub fn _burn(token_id: u64, keys: TokenKeys) {}

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
    let message = params.to_message(admin_treasury, admin_fee, Identity::ContractId(contract_id()));
    let recovered = ec_recover_evm_address(signature, message).unwrap();

    require(signer == recovered, TokenError::InvalidSignature);
}

// NOTE unfortunately I need to explicitly write out the map type, otherwise the compiler cries
// that there's no method `get` found for `StorageKey<TokenIdByAddressMap>`
#[storage(read)]
pub fn _has_claimed_by_address(
    user: Identity,
    guild_id: u64,
    action: GuildAction,
    key: StorageKey<StorageMap<Identity, StorageMap<u64, StorageMap<GuildAction, u64>>>>,
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
