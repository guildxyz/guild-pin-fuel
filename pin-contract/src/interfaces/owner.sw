library;

use ownership::Ownership;
use src_5::{AccessError, State};

use std::vm::evm::evm_address::EvmAddress;

abi OwnerControl {
    #[storage(read, write)]
    fn set_owner(owner: Identity);
    #[storage(read, write)]
    fn set_signer(signer: EvmAddress);
    #[storage(read, write)]
    fn set_treasury(treasury: Identity);
    #[storage(read, write)]
    fn set_fee(fee: u64);
}

abi OwnerInfo {
    #[storage(read)]
    fn owner() -> State;
    #[storage(read)]
    fn signer() -> EvmAddress;
    #[storage(read)]
    fn treasury() -> Identity;
    #[storage(read)]
    fn fee() -> u64;
}

pub struct OwnerChanged {
    old: Identity,
    new: Identity,
}

pub struct SignerChanged {
    old: EvmAddress,
    new: EvmAddress,
}

pub struct TreasuryChanged {
    old: Identity,
    new: Identity,
}

pub struct FeeChanged {
    old: u64,
    new: u64,
}

#[storage(read, write)]
pub fn _set_owner(owner: Identity, owner_key: StorageKey<Ownership>) {
    let caller = _only_owner(owner_key);
    owner_key.write(Ownership::initialized(owner));
    log(OwnerChanged {
        old: caller,
        new: owner,
    });
}

#[storage(read, write)]
pub fn _set_signer(
    signer: EvmAddress,
    key: StorageKey<b256>,
    owner_key: StorageKey<Ownership>,
) {
    let _caller = _only_owner(owner_key);
    let old_signer = key.read();
    key.write(signer.into());
    log(SignerChanged {
        old: EvmAddress::from(old_signer),
        new: signer,
    });
}

#[storage(read, write)]
pub fn _set_treasury(
    treasury: Identity,
    key: StorageKey<Identity>,
    owner_key: StorageKey<Ownership>,
) {
    let _caller = _only_owner(owner_key);
    let old_treasury = key.read();
    key.write(treasury);
    log(TreasuryChanged {
        old: old_treasury,
        new: treasury,
    });
}

#[storage(read, write)]
pub fn _set_fee(
    fee: u64,
    key: StorageKey<u64>,
    owner_key: StorageKey<Ownership>,
) {
    let _caller = _only_owner(owner_key);
    let old_fee = key.read();
    key.write(fee);
    log(FeeChanged {
        old: old_fee,
        new: fee,
    });
}

#[storage(read)]
pub fn _owner(key: StorageKey<Ownership>) -> State {
    key.read().state
}

#[storage(read)]
pub fn _signer(key: StorageKey<b256>) -> EvmAddress {
    EvmAddress::from(key.read())
}

#[storage(read)]
pub fn _treasury(key: StorageKey<Identity>) -> Identity {
    key.read()
}

#[storage(read)]
pub fn _fee(key: StorageKey<u64>) -> u64 {
    key.read()
}

// NOTE this implicitly checks whether the contract has been initialized
#[storage(read)]
fn _only_owner(key: StorageKey<Ownership>) -> Identity {
    // NOTE built-in storage.owner.only_owner() doesn't work compiler cannot find the method...
    //
    // anyways, at least we can modify this to return the msg_sender() as well
    let caller = msg_sender().unwrap();
    require(
        _owner(key) == State::Initialized(caller),
        AccessError::NotOwner,
    );
    caller
}
