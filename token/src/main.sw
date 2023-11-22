contract;

// TODO events, errors, metadata

mod errors;
mod events;

use ::events::PinMinted;
use ::errors::Error;

use ownership::Ownership;
use src_20::SRC20;
use src_3::SRC3;
use src_5::{AccessError, SRC5, State};
use std::{
    call_frames::{
        contract_id,
        msg_asset_id,
    },
    constants::ZERO_B256,
    context::msg_amount,
    hash::Hash,
    string::String,
    token::{
        burn,
        mint,
    },
};

configurable {
    NAME: str[9] = __to_str_array("Guild Pin"),
    SYMBOL: str[5] = __to_str_array("GUILD"),
    OWNER: Identity = Identity::Address(Address::from(ZERO_B256)),
}

storage {
    owner: Ownership = Ownership::uninitialized(),
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

abi GuildPinToken {
    #[storage(read, write)]
    fn initialize();
}

impl GuildPinToken for Contract {
    #[storage(read, write)]
    fn initialize() {
        // anyone can call this function but only once, until it's uninitialized
        require(storage.owner.read().state == State::Uninitialized, Error::AlreadyInitialized);
        // This is required because we don't necessarily know the owner such that it can be baked
        // into the code.
        //
        // However, we can set the owner via a configurable upon deployment, but it needs to be
        // written to storage as well. That's why we call this method and write the configurable
        // OWNER into storage here.
        storage.owner.write(Ownership::initialized(OWNER));
    }
}

impl SRC3 for Contract {
    #[storage(read, write)]
    fn mint(recipient: Identity, sub_id: SubId, amount: u64) {
        // NOTE this doesn't work for some reason (cannot find the method)
        //storage.owner.only_owner();
        require(
            storage
                .owner
                .read()
                .state == State::Initialized(msg_sender().unwrap()),
            AccessError::NotOwner,
        );
        require(amount == 1, Error::InvalidAmount);
        require(sub_id == ZERO_B256, Error::InvalidSubId);

        // use total minted for a unique id
        let pin_id = storage.total_minted.read();

        // this should never happen in theory but add check just to be safe
        //require(
        //    storage
        //        .owners
        //        .get(pin_id)
        //        .read()
        //        .is_none(),
        //        Error::AlreadyMinted,
        //);

        // mint only to this contract, otherwise users would be able to transfer the tokens
        //mint(ZERO_B256, amount);
        storage
            .balances
            .insert(recipient, storage.balances.get(recipient).read() + amount);
        storage.owners.insert(pin_id, Some(recipient));

        storage
            .total_supply
            .write(storage.total_supply.read() + amount);
        storage
            .total_minted
            .write(storage.total_supply.read() + amount);
        log(PinMinted {
            recipient,
            pin_id,
        });
    }

    #[storage(read, write)]
    fn burn(sub_id: SubId, amount: u64) {
        // NOTE we are using amount for the token id
        let pin_id = amount;
        require(sub_id == ZERO_B256, Error::InvalidSubId);
        require(
            msg_asset_id() == AssetId::default(contract_id()),
            Error::InvalidAssetId
        );
        let maybe_token_owner = storage.owners.get(pin_id).read();
        require(maybe_token_owner.is_some(), AccessError::NotOwner);
        let token_owner = maybe_token_owner.unwrap();
        require(msg_sender().unwrap() == token_owner, AccessError::NotOwner);

        let balance = storage.balances.get(token_owner).read();
        storage.balances.insert(token_owner, balance - 1);
        // Decrement total supply of the asset and burn.
        storage.total_supply.write(storage.total_supply.read() - 1);
        storage.owners.insert(pin_id, None);

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
