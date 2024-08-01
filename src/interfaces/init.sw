library;

use sway_libs::ownership::*;
use standards::src5::State;

use std::vm::evm::evm_address::EvmAddress;

pub enum InitError {
    AlreadyInitialized: (),
    NotInitialized: (),
}

pub struct ContractInitialized {
    pub owner: Identity,
    pub signer: EvmAddress,
    pub treasury: Identity,
    pub fee: u64,
}

pub struct InitKeys {
    pub signer: StorageKey<b256>,
    pub treasury: StorageKey<Identity>,
    pub fee: StorageKey<u64>,
}

abi Initialize {
    #[storage(read, write)]
    fn initialize();
}

#[storage(read, write)]
pub fn _initialize(params: ContractInitialized, keys: InitKeys) {
    // anyone can call this function but only once, until it's uninitialized
    require(
        _owner() == State::Uninitialized,
        InitError::AlreadyInitialized,
    );
    initialize_ownership(params.owner);
    keys.treasury.write(params.treasury);
    keys.signer.write(params.signer.into());
    keys.fee.write(params.fee);
    log(params);
}

#[storage(read)]
pub fn _initialized() {
    let initialized = match _owner() {
        State::Initialized(_) => true,
        _ => false,
    };
    require(initialized, InitError::NotInitialized);
}
