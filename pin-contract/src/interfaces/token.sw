library;

use ::common::*;
use ::interfaces::init::InitKeys;

use std::b512::B512;
use std::call_frames::contract_id;
use std::hash::Hash;
use std::vm::evm::ecr::ec_recover_evm_address;
use std::vm::evm::evm_address::EvmAddress;

pub enum TokenError {
    InvalidSignature: (),
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
    fn has_claimed_by_id(user_id: u64, guils_id: u64, action: GuildAction) -> bool;
}

#[storage(read, write)]
pub fn _claim(
    params: PinDataParams,
    signature: B512,
    token_keys: TokenKeys,
    init_keys: InitKeys,
) {}

#[storage(read, write)]
pub fn _burn(token_id: u64, keys: TokenKeys) {}

#[storage(read)]
pub fn _check_signature(signature: B512, params: PinDataParams, init_keys: InitKeys) {
    let signer = EvmAddress::from(init_keys.signer.read());
    let treasury = init_keys.treasury.read();
    let fee = init_keys.fee.read();
    let message = params.to_message(treasury, fee, Identity::ContractId(contract_id()));
    let recovered = ec_recover_evm_address(signature, message).unwrap();

    require(signer == recovered, TokenError::InvalidSignature);
}
