library;

use ownership::Ownership;
use src_5::State;

use std::vm::evm::evm_address::EvmAddress;

pub enum InitError {
    AlreadyInitialized: (),
    NotInitialized: (),
}

pub struct ContractInitialized {
    owner: Identity,
    signer: EvmAddress,
    treasury: Identity,
    fee: u64,
}

pub struct InitKeys {
    owner: StorageKey<Ownership>,
    signer: StorageKey<b256>,
    treasury: StorageKey<Identity>,
    fee: StorageKey<u64>,
}

abi Initialize {
    #[storage(read, write)]
    fn initialize();
}

#[storage(read, write)]
pub fn _initialize(params: ContractInitialized, keys: InitKeys) {
    // anyone can call this function but only once, until it's uninitialized
    require(
        keys.owner
            .read()
            .state == State::Uninitialized,
        InitError::AlreadyInitialized,
    );
    // NOTE cannot check this because current std version can't find this
    // method IDK
    //let signer_bytes = signer.to_le_bytes();
    //require(!&signer_bytes[20..].into_iter().any(|byte| byte != 0), InitError::InvalidSigner);

    // Initialization is required because we don't necessarily know the owner such that it can be
    // baked into the code.
    //
    // However, we can set the owner via a configurable upon deployment, but it needs to be
    // written to storage as well. That's why we call this method and write the configurable
    // OWNER into storage here.
    keys.owner.write(Ownership::initialized(params.owner));
    keys.treasury.write(params.treasury);
    keys.signer.write(params.signer.into());
    keys.fee.write(params.fee);
    log(params);
}

#[storage(read)]
pub fn _initialized(owner_key: StorageKey<Ownership>) {
    let initialized = match owner_key.read().state {
        State::Initialized(_) => true,
        _ => false,
    };
    require(initialized, InitError::NotInitialized);
}
