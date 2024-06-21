library;

use sway_libs::ownership::{transfer_ownership, only_owner};

use std::vm::evm::evm_address::EvmAddress;

abi OnlyOwner {
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
    fn signer() -> b256;
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
pub fn _set_owner(new_owner: Identity) {
    transfer_ownership(new_owner);
}

#[storage(read, write)]
pub fn _set_signer(
    signer: EvmAddress,
    key: StorageKey<b256>,
) {
    only_owner();
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
) {
    only_owner();
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
) {
    only_owner();
    let old_fee = key.read();
    key.write(fee);
    log(FeeChanged {
        old: old_fee,
        new: fee,
    });
}

#[storage(read)]
pub fn _signer(key: StorageKey<b256>) -> b256 {
    // NOTE cannot return EvmAddress, because it gets added to the abi as a () type
    key.read()
}

#[storage(read)]
pub fn _treasury(key: StorageKey<Identity>) -> Identity {
    key.read()
}

#[storage(read)]
pub fn _fee(key: StorageKey<u64>) -> u64 {
    key.read()
}
