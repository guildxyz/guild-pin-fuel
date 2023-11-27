contract;

mod errors;
mod events;

use ::errors::TreasuryError;
use ::events::{ContractInitialized, FeeSet, OwnerSet, TreasurySet};

use ownership::Ownership;
use src_5::{AccessError, SRC5, State};
use std::constants::ZERO_B256;

configurable {
    OWNER: Identity = Identity::Address(Address::from(ZERO_B256)),
    TREASURY: Identity = Identity::Address(Address::from(ZERO_B256)),
    FEE: u64 = 0,
}

storage {
    owner: Ownership = Ownership::uninitialized(),
    treasury: Identity = TREASURY,
    fee: u64 = FEE,
}

abi GuildPinTreasury {
    #[storage(read, write)]
    fn initialize();
    #[storage(read, write)]
    fn set_owner(owner: Identity);
    #[storage(read, write)]
    fn set_fee(fee: u64);
    #[storage(read, write)]
    fn set_treasury(treasury: Identity);
    #[storage(read)]
    fn fee() -> u64;
    #[storage(read)]
    fn treasury() -> Identity;
}

impl GuildPinTreasury for Contract {
    #[storage(read, write)]
    fn initialize() {
        // anyone can call this function, but only once
        require(storage.owner.read().state == State::Uninitialized, TreasuryError::AlreadyInitialized);
        storage.treasury.write(TREASURY);
        storage.fee.write(FEE);
        storage.owner.write(Ownership::initialized(OWNER));

        log(ContractInitialized {
            owner: OWNER,
            treasury: TREASURY,
            fee: FEE,
        });
    }
    #[storage(read, write)]
    fn set_owner(owner: Identity) {
        let old_owner = msg_sender().unwrap();
        _only_owner(old_owner);
        storage.owner.write(Ownership::initialized(owner));
        log(OwnerSet {
            old: old_owner,
            new: owner,
        })
    }

    #[storage(read, write)]
    fn set_fee(fee: u64) {
        _only_owner(msg_sender().unwrap());
        let old_fee = storage.fee.read();
        storage.fee.write(fee);
        log(FeeSet {
            old: old_fee,
            new: fee,
        })
    }

    #[storage(read, write)]
    fn set_treasury(treasury: Identity) {
        _only_owner(msg_sender().unwrap());
        let old_treasury = storage.treasury.read();
        storage.treasury.write(treasury);
        log(TreasurySet {
            old: old_treasury,
            new: treasury,
        })
    }

    #[storage(read)]
    fn fee() -> u64 {
        storage.fee.read()
    }

    #[storage(read)]
    fn treasury() -> Identity {
        storage.treasury.read()
    }
}

impl SRC5 for Contract {
    #[storage(read)]
    fn owner() -> State {
        storage.owner.read().state
    }
}

#[storage(read)]
fn _only_owner(caller: Identity) {
    // NOTE this doesn't work for some reason (cannot find the method)
    //storage.owner.only_owner();
    require(storage.owner.read().state == State::Initialized(caller), AccessError::NotOwner);
}
