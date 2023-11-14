contract;

use src_3::SRC3;
use src_20::SRC20;
use std::{
    call_frames::{
        contract_id,
        msg_asset_id,
    },
    constants::ZERO_B256,
    context::msg_amount,
    string::String,
    token::{
        burn,
        mint_to,
    },
};

configurable {
    NAME: str[9] = __to_str_array("Guild Pin"),
    SYMBOL: str[5] = __to_str_array("GUILD"),
}

storage {
    //dummy: Dummy = Dummy::new(),
    total_supply: u64 = 0,
}

impl SRC3 for Contract {
    #[storage(read, write)]
    fn mint(recipient: Identity, sub_id: SubId, amount: u64) {
        require(amount == 1, "Minting multiple assets is forbidden");
        require(sub_id == ZERO_B256, "Incorrect Sub Id");

        // Increment total supply of the asset and mint to the recipient.
        storage.total_supply.write(amount + storage.total_supply.read());
        mint_to(recipient, ZERO_B256, amount);
    }
    #[storage(read, write)]
    fn burn(sub_id: SubId, amount: u64) {
        require(sub_id == ZERO_B256, "Incorrect Sub Id");
        require(msg_amount() >= amount, "Incorrect amount provided");
        require(msg_asset_id() == AssetId::default(contract_id()), "Incorrect asset provided");

        // Decrement total supply of the asset and burn.
        storage.total_supply.write(storage.total_supply.read() - amount);
        burn(ZERO_B256, amount);
    }
}

impl SRC20 for Contract {
    #[storage(read)]
    fn total_assets() -> u64 {
        1
    }

    #[storage(read)]
    fn total_supply(asset: AssetId) -> Option<u64> {
        if asset == AssetId::default(contract_id()) {
            Some(storage.total_supply.read())
        } else {
            None
        }
    }

    #[storage(read)]
    fn name(asset: AssetId) -> Option<String> {
        if asset == AssetId::default(contract_id()) {
            Some(String::from_ascii_str(from_str_array(NAME)))
        } else {
            None
        }
    }

    #[storage(read)]
    fn symbol(asset: AssetId) -> Option<String> {
        if asset == AssetId::default(contract_id()) {
            Some(String::from_ascii_str(from_str_array(SYMBOL)))
        } else {
            None
        }
    }

    #[storage(read)]
    fn decimals(asset: AssetId) -> Option<u8> {
        if asset == AssetId::default(contract_id()) {
            Some(0)
        } else {
            None
        }
    }
}
