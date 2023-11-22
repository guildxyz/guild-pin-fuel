contract;

mod errors;
mod events;

use ::errors::TreasuryError;
use ::events::ContractInitialized;

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
}

impl GuildPinTreasury for Contract {
    #[storage(read, write)]
    fn initialize() {
        require(
            storage
                .owner
                .read()
                .state == State::Uninitialized,
            TreasuryError::AlreadyInitialized,
        );
        storage.treasury.write(TREASURY);
        storage.fee.write(FEE);
        storage.owner.write(Ownership::initialized(OWNER));

        log(ContractInitialized {
            owner: OWNER,
            treasury: TREASURY,
            fee: FEE,
        });
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
    require(
        storage
            .owner
            .read()
            .state == State::Initialized(caller),
        AccessError::NotOwner,
    );
}
