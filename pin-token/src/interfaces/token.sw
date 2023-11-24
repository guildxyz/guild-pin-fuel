library;

use std::call_frames::contract_id;
use std::constants::ZERO_B256;
use std::hash::Hash;
use std::token::{burn, mint};

pub enum TokenError {
    AlreadyBurned: (),
    InvalidMinter: (),
    NotPinOwner: (),
    PinIdDoesNotExist: (),
}

pub struct PinMinted {
    recipient: Identity,
    pin_id: u64,
}

pub struct PinBurned {
    pin_owner: Identity,
    pin_id: u64,
}

// Only the contract itself may call this function
#[storage(read, write)]
pub fn _mint(
    recipient: Identity,
    total_minted_key: StorageKey<u64>,
    total_supply_key: StorageKey<u64>,
    balances_key: StorageKey<StorageMap<Identity, u64>>,
    owners_key: StorageKey<StorageMap<u64, Option<Identity>>>,
) {
    // only the contract itself may call this function
    require(
        msg_sender()
            .unwrap() == Identity::ContractId(contract_id()),
        TokenError::InvalidMinter,
    );
    // we can only mint a single token per call
    // mint only to this contract, otherwise users would be able to transfer the tokens
    mint(ZERO_B256, 1);
    // use total minted for a unique pin id
    let pin_id = total_minted_key.read();
    // assign recipient as owner to the pin_id
    owners_key.insert(pin_id, Some(recipient));
    // increment balance of recipient
    let balance = balances_key.get(recipient).try_read().unwrap_or(0);
    balances_key.insert(recipient, balance + 1);
    // increment total supply
    total_supply_key.write(total_supply_key.read() + 1);
    // increment total minted
    total_minted_key.write(total_minted_key.read() + 1);
    // emit event
    log(PinMinted {
        recipient,
        pin_id,
    });
}

#[storage(read, write)]
pub fn _burn(
    pin_id: u64,
    total_supply_key: StorageKey<u64>,
    balances_key: StorageKey<StorageMap<Identity, u64>>,
    owners_key: StorageKey<StorageMap<u64, Option<Identity>>>,
) {
    let pin_owner = match owners_key.get(pin_id).try_read() {
        Some(Some(pin_owner)) => {
            require(msg_sender().unwrap() == pin_owner, TokenError::NotPinOwner);
            pin_owner
        },
        Some(None) => {
            require(false, TokenError::AlreadyBurned);
            revert(12)
        },
        None => {
            require(false, TokenError::PinIdDoesNotExist);
            revert(13)
        }
    };

    let balance = balances_key.get(pin_owner).read();
    balances_key.insert(pin_owner, balance - 1);
    // Decrement total supply of the asset and burn.
    total_supply_key.write(total_supply_key.read() - 1);
    owners_key.insert(pin_id, None);

    burn(ZERO_B256, 1);

    log(PinBurned {
        pin_owner,
        pin_id,
    })
}
