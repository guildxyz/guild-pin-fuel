contract;

// TODO metadata SRC-7

mod interfaces;

use ::interfaces::info::*;
use ::interfaces::init::*;
use ::interfaces::owner::*;
use ::interfaces::src20::*;
use ::interfaces::token::*;

use ownership::Ownership;
use src_20::SRC20;
use src_3::SRC3;
use src_5::{AccessError, SRC5, State};

use guild_pin_common::TokenUri;
use std::constants::ZERO_B256;
use std::hash::Hash;
use std::string::String;
use std::vm::evm::evm_address::EvmAddress;

// NOTE EvmAddress cannot be const initialized 
// https://github.com/FuelLabs/sway/issues/4967
//
// I also cannot use [u8; 20] because arrays in storage
// has not been implemented yet
configurable {
    NAME: str[9] = __to_str_array("Guild Pin"),
    SYMBOL: str[5] = __to_str_array("GUILD"),
    OWNER: Identity = Identity::Address(Address::from(ZERO_B256)),
    TREASURY: Identity = Identity::ContractId(ContractId::from(ZERO_B256)),
    SIGNER: b256 = ZERO_B256,
    SIGNATURE_VALIDITY: u64 = 3600, // seconds
}

storage {
    owner: Ownership = Ownership::uninitialized(),
    treasury: Identity = Identity::ContractId(ContractId::from(ZERO_B256)),
    signer: b256 = ZERO_B256,
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
    /// Dummy key to make warnings disappear
    warning: bool = false,
}

impl Initialize for Contract {
    #[storage(read, write)]
    fn initialize() {
        _initialize(OWNER, storage.owner, TREASURY, storage.treasury, SIGNER, storage.signer)
    }
}

impl SRC3 for Contract {
    #[storage(read, write)]
    fn mint(recipient: Identity, _sub_id: SubId, _amount: u64) {
        _mint(recipient, storage.total_minted, storage.total_supply, storage.balances, storage.owners)
    }

    #[storage(read, write)]
    fn burn(_sub_id: SubId, pin_id: u64) {
        // NOTE we are using amount as pin_id
        _burn(pin_id, storage.total_supply, storage.balances, storage.owners)
    }
}

impl SRC20 for Contract {
    #[storage(read)]
    fn total_assets() -> u64 {
        let _ = storage.warning.read();
        _total_assets()
    }

    #[storage(read)]
    fn total_supply(asset: AssetId) -> Option<u64> {
        _total_supply(asset, storage.total_supply)
    }

    #[storage(read)]
    fn name(asset: AssetId) -> Option<String> {
        let _ = storage.warning.read();
        _name(asset, NAME)
    }

    #[storage(read)]
    fn symbol(asset: AssetId) -> Option<String> {
        let _ = storage.warning.read();
        _symbol(asset, SYMBOL)
    }

    #[storage(read)]
    fn decimals(asset: AssetId) -> Option<u8> {
        let _ = storage.warning.read();
        _decimals(asset)
    }
}

impl SRC5 for Contract {
    #[storage(read)]
    fn owner() -> State {
        _owner(storage.owner)
    }
}

impl SetOwner for Contract {
    #[storage(read, write)]
    fn set_owner(owner: Identity) {
        _set_owner(owner, storage.owner)
    }
}
