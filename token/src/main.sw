contract;


use ownership::Ownership;
use src_20::SRC20;
use src_3::SRC3;
use src_5::{SRC5, State};
use std::{
    call_frames::{contract_id, msg_asset_id},
    constants::ZERO_B256,
    context::msg_amount,
    hash::Hash,
    string::String,
    token::{burn, mint},
};

configurable {
    NAME: str[9] = __to_str_array("Guild Pin"),
    SYMBOL: str[5] = __to_str_array("GUILD"),
}

storage {
    owner: Ownership = Ownership::initialized(Identity::Address(Address::from(0x0000000000000000000000000000000000000000000000000000000000000000))),
    /// Quick O(1) access to an user's balance
    balances: StorageMap<Identity, u64> = StorageMap {},
    /// Returns the owner of a token with a given ID. None, if
    /// the token has already been burned.
    owners: StorageMap<u64, Option<Identity>> = StorageMap {},
    // TODO
    //metadata: StorageMap<u64, TokenMetadata> = StorageMap {},
    /// Incremented upon mint, decremented upon burn
    total_supply: u64 = 0,
    /// Only incremented
    total_minted: u64 = 0,
}

impl SRC3 for Contract {
    #[storage(read, write)]
    fn mint(recipient: Identity, sub_id: SubId, amount: u64) {
        storage.owner.only_owner();
        require(amount == 1, "Minting multiple assets is forbidden");
        require(sub_id == ZERO_B256, "Incorrect Sub Id");

        // Increment total supply of the asset and mint to the recipient.
        storage
            .total_supply
            .write(amount + storage.total_supply.read());
        // mint only to this contract, otherwise users would be able to transfer the tokens
        mint(ZERO_B256, amount);
    }
    #[storage(read, write)]
    fn burn(sub_id: SubId, amount: u64) {
        require(sub_id == ZERO_B256, "Incorrect Sub Id");
        require(msg_amount() >= amount, "Incorrect amount provided");
        require(
            msg_asset_id() == AssetId::default(contract_id()),
            "Incorrect asset provided",
        );

        // Decrement total supply of the asset and burn.
        storage
            .total_supply
            .write(storage.total_supply.read() - amount);
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

impl SRC5 for Contract {
    #[storage(read)]
    fn owner() -> State {
        storage.owner.read().state
    }
}
