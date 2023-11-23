contract;

// TODO metadata SRC-7

mod errors;
mod events;
mod interfaces;

use ::events::{ContractInitialized, OwnerSet, PinBurned, PinMinted};
use ::errors::TokenError;
use ::interfaces::src5::{only_owner, SetOwner};

use ownership::Ownership;
use src_20::SRC20;
use src_3::SRC3;
use src_5::{AccessError, SRC5, State};

use guild_pin_common::TokenUri;
use std::{
    call_frames::{
        contract_id,
        msg_asset_id,
    },
    constants::ZERO_B256,
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
    /// Metadata attached to a
    metadata: StorageMap<u64, TokenUri> = StorageMap {},
    /// Incremented upon mint, decremented upon burn
    total_supply: u64 = 0,
    /// Only incremented
    total_minted: u64 = 0,
}

abi GuildPinToken {
    #[storage(read, write)]
    fn initialize();
    #[storage(read, write)]
    fn set_metadata(metadata: TokenUri);
    #[storage(read)]
    fn balance(of: Identity) -> u64;
    #[storage(read)]
    fn pin_owner(pin_id: u64) -> Option<Identity>;
    #[storage(read)]
    fn total_minted() -> u64;
}

impl GuildPinToken for Contract {
    #[storage(read, write)]
    fn initialize() {
        // anyone can call this function but only once, until it's uninitialized
        require(
            storage
                .owner
                .read()
                .state == src_5::State::Uninitialized,
            TokenError::AlreadyInitialized,
        );
        // This is required because we don't necessarily know the owner such that it can be baked
        // into the code.
        //
        // However, we can set the owner via a configurable upon deployment, but it needs to be
        // written to storage as well. That's why we call this method and write the configurable
        // OWNER into storage here.
        storage.owner.write(Ownership::initialized(OWNER));
        log(ContractInitialized { owner: OWNER })
    }

    #[storage(read, write)]
    fn set_metadata(metadata: TokenUri) {
        revert(0)
    }

    #[storage(read)]
    fn balance(of: Identity) -> u64 {
        storage.balances.get(of).try_read().unwrap_or(0)
    }

    #[storage(read)]
    fn pin_owner(pin_id: u64) -> Option<Identity> {
        let maybe_owner = storage.owners.get(pin_id).try_read();
        if let Some(owner) = maybe_owner {
            owner
        } else {
            require(false, TokenError::PinIdDoesNotExist);
            revert(155)
        }
    }

    #[storage(read)]
    fn total_minted() -> u64 {
        storage.total_minted.read()
    }
}

impl SRC3 for Contract {
    #[storage(read, write)]
    fn mint(recipient: Identity, sub_id: SubId, amount: u64) {
        only_owner(storage.owner);
        require(amount == 1, TokenError::InvalidAmount);
        require(sub_id == ZERO_B256, TokenError::InvalidSubId);
        require(
            msg_asset_id() == AssetId::default(contract_id()),
            TokenError::InvalidAssetId,
        );

        // use total minted for a unique id
        let pin_id = storage.total_minted.read();

        // this should never happen in theory but add check just to be safe
        require(
            storage
                .owners
                .get(pin_id)
                .try_read()
                .is_none(),
            TokenError::AlreadyMinted,
        );

        // mint only to this contract, otherwise users would be able to transfer the tokens
        mint(ZERO_B256, amount);

        // increment balance of recipient
        let balance = storage.balances.get(recipient).try_read().unwrap_or(0);
        storage.balances.insert(recipient, balance + amount);
        // assign recipient as owner to the pin_id
        storage.owners.insert(pin_id, Some(recipient));
        // increment total supply
        storage
            .total_supply
            .write(storage.total_supply.read() + amount);
        // increment total minted
        storage
            .total_minted
            .write(storage.total_minted.read() + amount);
        // emit event
        log(PinMinted {
            recipient,
            pin_id,
        });
    }

    #[storage(read, write)]
    fn burn(sub_id: SubId, amount: u64) {
        // NOTE we are using amount for the token id
        let pin_id = amount;
        require(sub_id == ZERO_B256, TokenError::InvalidSubId);
        require(
            msg_asset_id() == AssetId::default(contract_id()),
            TokenError::InvalidAssetId,
        );
        let pin_owner = match storage.owners.get(pin_id).try_read() {
            Some(Some(pin_owner)) => {
                require(msg_sender().unwrap() == pin_owner, AccessError::NotOwner);
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

        let balance = storage.balances.get(pin_owner).read();
        storage.balances.insert(pin_owner, balance - 1);
        // Decrement total supply of the asset and burn.
        storage.total_supply.write(storage.total_supply.read() - 1);
        storage.owners.insert(pin_id, None);

        burn(ZERO_B256, amount);

        log(PinBurned {
            pin_owner,
            pin_id,
        })
    }
}

impl SRC20 for Contract {
    #[storage(read)]
    fn total_assets() -> u64 {
        interfaces::src20::total_assets()
    }

    #[storage(read)]
    fn total_supply(asset: AssetId) -> Option<u64> {
        interfaces::src20::total_supply(asset, storage.total_supply)
    }

    #[storage(read)]
    fn name(asset: AssetId) -> Option<String> {
        interfaces::src20::name(asset, NAME)
    }

    #[storage(read)]
    fn symbol(asset: AssetId) -> Option<String> {
        interfaces::src20::symbol(asset, SYMBOL)
    }

    #[storage(read)]
    fn decimals(asset: AssetId) -> Option<u8> {
        interfaces::src20::decimals(asset)
    }
}

impl SRC5 for Contract {
    #[storage(read)]
    fn owner() -> State {
        interfaces::src5::owner(storage.owner)
    }
}

impl SetOwner for Contract {
    #[storage(read, write)]
    fn set_owner(owner: Identity) {
        interfaces::src5::set_owner(owner, storage.owner)
    }
}
