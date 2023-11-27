library;

use ownership::Ownership;
use src_5::State;

use std::vm::evm::evm_address::EvmAddress;

pub enum InitError {
    AlreadyInitialized: (),
    InvalidTreasury: (),
    InvalidSigner: (),
}

pub struct ContractInitialized {
    owner: Identity,
    treasury: Identity,
    signer: EvmAddress,
}

abi Initialize {
    #[storage(read, write)]
    fn initialize();
}

#[storage(read, write)]
pub fn _initialize(
    owner: Identity,
    owner_key: StorageKey<Ownership>,
    treasury: Identity,
    treasury_key: StorageKey<Identity>,
    signer: b256,
    signer_key: StorageKey<b256>,
) {
    // anyone can call this function but only once, until it's uninitialized
    require(owner_key.read().state == State::Uninitialized, InitError::AlreadyInitialized);
    require(treasury.is_contract_id(), InitError::InvalidTreasury);
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
    owner_key.write(Ownership::initialized(owner));
    treasury_key.write(treasury);
    signer_key.write(signer);
    log(ContractInitialized {
        owner,
        treasury,
        signer: EvmAddress::from(signer),
    })
}
