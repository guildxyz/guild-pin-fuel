contract;

mod common;
mod interfaces;

use ::common::action::GuildAction;
use ::common::claim::ClaimParameters;
use ::common::pin::PinData;
use ::common::utils::parse_u64;
use ::common::*;
use ::interfaces::init::*;
use ::interfaces::metadata::*;
use ::interfaces::owner::*;
use ::interfaces::src20::*;
use ::interfaces::token::*;
use sway_libs::ownership::*;
use standards::src20::SRC20;
use standards::src5::{SRC5, State};
use standards::src7::{Metadata, SRC7};

use std::b512::B512;
use std::constants::ZERO_B256;
use std::hash::Hash;
use std::string::String;
use std::vm::evm::evm_address::EvmAddress;

configurable {
    NAME: str[9] = __to_str_array("Guild Pin"),
    SYMBOL: str[5] = __to_str_array("GUILD"),
    OWNER: Identity = Identity::Address(Address::from(ZERO_B256)),
    SIGNER: b256 = ZERO_B256,
    SIGNATURE_VALIDITY_PERIOD: u64 = 3600,
    TREASURY: Identity = Identity::ContractId(ContractId::from(ZERO_B256)),
    FEE: u64 = 0,
}

storage {
    /// Evm address of the guild-backend signer wallet
    signer: b256 = ZERO_B256,
    /// Treasury address receiving minting fees
    treasury: Identity = Identity::Address(Address::from(ZERO_B256)),
    /// Fee collected upon claiming a pin
    fee: u64 = 0,
    /// Map: pin_id -> metadata
    metadata: StorageMap<u64, PinData> = StorageMap {},
    /// Map: address -> pin_balance (increment upon claim, decrement upon burn)
    balances: BalancesMap = StorageMap {},
    /// Map: pin_id -> maybe_owner (None if burned)
    pin_owners: OwnersMap = StorageMap {},
    /// Map: (address + guild_id + guild_action) -> pin_id
    token_id_by_address: TokenIdByAddressMap = StorageMap {},
    /// Map: (user_id + guild_id + guild_action) -> pin_id
    token_id_by_user_id: TokenIdByUserIdMap = StorageMap {},
    /// Only incremented
    total_minted_per_guild: TotalMintedPerGuildMap = StorageMap {},
    /// Map: (address + token index) -> pin_id
    token_of_owner_by_index: TokenOfOwnerByIndexMap = StorageMap {},
    /// Only incremented
    total_minted: u64 = 0,
    /// Incremented upon successful claim, decremented upon successful burn
    total_supply: u64 = 0,
    /// Dummy key to make warnings disappear
    warning: bool = false,
}

impl Initialize for Contract {
    #[storage(read, write)]
    fn initialize() {
        let params = ContractInitialized {
            owner: OWNER,
            signer: EvmAddress::from(SIGNER),
            treasury: TREASURY,
            fee: FEE,
        };
        let keys = InitKeys {
            signer: storage.signer,
            treasury: storage.treasury,
            fee: storage.fee,
        };
        _initialize(params, keys);
    }
}

impl OnlyOwner for Contract {
    #[storage(read, write)]
    fn set_owner(owner: Identity) {
        _set_owner(owner)
    }
    #[storage(read, write)]
    fn set_signer(signer: EvmAddress) {
        _set_signer(signer, storage.signer)
    }
    #[storage(read, write)]
    fn set_treasury(treasury: Identity) {
        _set_treasury(treasury, storage.treasury)
    }
    #[storage(read, write)]
    fn set_fee(fee: u64) {
        _set_fee(fee, storage.fee)
    }
}

impl OwnerInfo for Contract {
    #[storage(read)]
    fn signer() -> b256 {
        _signer(storage.signer)
    }
    #[storage(read)]
    fn treasury() -> Identity {
        _treasury(storage.treasury)
    }
    #[storage(read)]
    fn fee() -> u64 {
        _fee(storage.fee)
    }
}

impl PinToken for Contract {
    #[payable]
    #[storage(read, write)]
    fn claim(params: ClaimParameters, signature: B512) {
        let token_keys = TokenKeys {
            metadata: storage.metadata,
            balances: storage.balances,
            pin_owners: storage.pin_owners,
            token_id_by_address: storage.token_id_by_address,
            token_id_by_user_id: storage.token_id_by_user_id,
            total_minted_per_guild: storage.total_minted_per_guild,
            total_minted: storage.total_minted,
            total_supply: storage.total_supply,
            token_of_address_by_index: storage.token_of_owner_by_index,
        };

        let init_keys = InitKeys {
            signer: storage.signer,
            treasury: storage.treasury,
            fee: storage.fee,
        };
        _claim(
            params,
            signature,
            SIGNATURE_VALIDITY_PERIOD,
            token_keys,
            init_keys,
        );
    }

    #[storage(read, write)]
    fn burn(_pin_id: u64) {
        // NOTE temporarily removed
        //let token_keys = TokenKeys {
        //    metadata: storage.metadata,
        //    balances: storage.balances,
        //    pin_owners: storage.pin_owners,
        //    token_id_by_address: storage.token_id_by_address,
        //    token_id_by_user_id: storage.token_id_by_user_id,
        //    total_minted_per_guild: storage.total_minted_per_guild,
        //    total_minted: storage.total_minted,
        //    total_supply: storage.total_supply,
        //    token_of_address_by_index: storage.token_of_address_by_index,
        //};
        //_burn(pin_id, token_keys)
        log("burning tokens is not allowed");
    }
}

impl PinInfo for Contract {
    #[storage(read)]
    fn balance_of(id: Address) -> u64 {
        _balance_of(id, storage.balances)
    }
    #[storage(read)]
    fn pin_owner(pin_id: u64) -> Option<Address> {
        _pin_owner(pin_id, storage.pin_owners)
    }
    #[storage(read)]
    fn total_minted() -> u64 {
        storage.total_minted.read()
    }

    #[storage(read)]
    fn total_minted_per_guild(guild_id: u64) -> u64 {
        _total_minted_per_guild(guild_id, storage.total_minted_per_guild)
    }

    #[storage(read)]
    fn pin_id_by_address(user: Address, guild_id: u64, action: GuildAction) -> Option<u64> {
        _pin_id_by_address(user, guild_id, action, storage.token_id_by_address)
    }

    #[storage(read)]
    fn pin_id_by_user_id(user_id: u64, guild_id: u64, action: GuildAction) -> Option<u64> {
        _pin_id_by_user_id(user_id, guild_id, action, storage.token_id_by_user_id)
    }

    #[storage(read)]
    fn token_of_owner_by_index(user: Address, index: u64) -> Option<u64> {
        _token_of_owner_by_index(user, index, storage.token_of_owner_by_index)
    }
}

impl SRC5 for Contract {
    #[storage(read)]
    fn owner() -> State {
        _owner() // from src5
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

impl SRC7 for Contract {
    #[storage(read)]
    fn metadata(asset_id: AssetId, key: String) -> Option<Metadata> {
        if asset_id != AssetId::default() {
            None
        } else {
            // damn there's no `map` on Option<T>
            if let Some(pin_id) = parse_u64(key) {
                Some(Metadata::String(_metadata(pin_id, storage.metadata)))
            } else {
                None
            }
        }
    }
}

impl PinMetadata for Contract {
    #[storage(read)]
    fn pin_metadata(pin_id: u64) -> String {
        _metadata(pin_id, storage.metadata)
    }

    #[storage(read)]
    fn encoded_metadata(pin_id: u64) -> String {
        _encoded_metadata(pin_id, storage.metadata)
    }
}
