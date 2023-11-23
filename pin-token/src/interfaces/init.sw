library;

use ownership::Ownership;
use src_5::State;

pub enum InitError {
    AlreadyInitialized: (),
}

pub struct ContractInitialized {
    owner: Identity,
}

abi Initialize {
    #[storage(read, write)]
    fn initialize();
}

#[storage(read, write)]
pub fn _initialize(owner: Identity, owner_key: StorageKey<Ownership>) {
    // anyone can call this function but only once, until it's uninitialized
    require(
        owner_key
            .read()
            .state == State::Uninitialized,
        InitError::AlreadyInitialized,
    );
    // This is required because we don't necessarily know the owner such that it can be baked
    // into the code.
    //
    // However, we can set the owner via a configurable upon deployment, but it needs to be
    // written to storage as well. That's why we call this method and write the configurable
    // OWNER into storage here.
    owner_key.write(Ownership::initialized(owner));
    log(ContractInitialized { owner })
}
