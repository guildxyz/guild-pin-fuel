library;

use ownership::Ownership;
use src_5::{AccessError, State};
use src_5::SRC5;

abi SetOwner {
    #[storage(read, write)]
    fn set_owner(owner: Identity);
}

pub struct OwnerSet {
    old: Identity,
    new: Identity,
}

#[storage(read, write)]
pub fn _set_owner(owner: Identity, key: StorageKey<Ownership>) {
    let caller = only_owner(key);
    key.write(Ownership::initialized(owner));
    log(OwnerSet {
        old: caller,
        new: owner,
    });
}

#[storage(read)]
pub fn _owner(key: StorageKey<Ownership>) -> State {
    key.read().state
}

#[storage(read)]
pub fn only_owner(key: StorageKey<Ownership>) -> Identity {
    // NOTE built-in storage.owner.only_owner() doesn't work
    // compiler cannot find the method...
    let caller = msg_sender().unwrap();
    require(
        _owner(key) == State::Initialized(caller),
        AccessError::NotOwner,
    );
    caller
}
