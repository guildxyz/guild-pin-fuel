contract;

mod common;
mod interfaces;

use ::common::*;
use ::interfaces::init::*;
use ::interfaces::owner::*;
use ::interfaces::token::*;

use ownership::Ownership;
use src_5::State;

use std::b512::B512;
use std::constants::ZERO_B256;
use std::hash::Hash;
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
    owner: Ownership = Ownership::uninitialized(),
    signer: b256 = ZERO_B256,
    treasury: Identity = Identity::Address(Address::from(ZERO_B256)),
    fee: u64 = FEE,
    metadata: StorageMap<u64, PinData> = StorageMap {},
    balances: BalancesMap = StorageMap {},
    pin_owners: OwnersMap = StorageMap {},
    token_id_by_address: TokenIdByAddressMap = StorageMap {},
    token_id_by_user_id: TokenIdByUserIdMap = StorageMap {},
    total_minted_per_guild: TotalMintedPerGuildMap = StorageMap {},
    total_minted: u64 = 0,
    total_supply: u64 = 0,
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
            owner: storage.owner,
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
        _set_owner(owner, storage.owner)
    }
    #[storage(read, write)]
    fn set_signer(signer: EvmAddress) {
        _set_signer(signer, storage.signer, storage.owner)
    }
    #[storage(read, write)]
    fn set_treasury(treasury: Identity) {
        _set_treasury(treasury, storage.treasury, storage.owner)
    }
    #[storage(read, write)]
    fn set_fee(fee: u64) {
        _set_fee(fee, storage.fee, storage.owner)
    }
}

impl OwnerInfo for Contract {
    #[storage(read)]
    fn owner() -> State {
        _owner(storage.owner)
    }
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
        };

        let init_keys = InitKeys {
            owner: storage.owner,
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
    fn burn(pin_id: u64) {
        let token_keys = TokenKeys {
            metadata: storage.metadata,
            balances: storage.balances,
            pin_owners: storage.pin_owners,
            token_id_by_address: storage.token_id_by_address,
            token_id_by_user_id: storage.token_id_by_user_id,
            total_minted_per_guild: storage.total_minted_per_guild,
            total_minted: storage.total_minted,
            total_supply: storage.total_supply,
        };
        _burn(pin_id, token_keys)
    }
}

impl PinInfo {
    #[storage(read)]
    fn balance_of(id: Address) -> u64 {
        storage.balances.get(id).read()
    }
    #[storage(read)]
    fn pin_owner(pin_id: u64) -> Option<Address> {
        storage.owners.get(pin_id).read()
    }
    #[storage(read)]
    fn total_minted() -> u64 {
        storage.total_minted.read()
    }
    #[storage(read)]
    fn has_claimed_by_address(user: Address, guild_id: u64, action: GuildAction) -> bool {
        _has_claimed_by_address(user, guild_id, action, storage.token_id_by_address)
    }

    #[storage(read)]
    fn has_claimed_by_user_id(user_id: u64, guils_id: u64, action: GuildAction) -> bool {
        _has_claimed_by_address(user, guild_id, action, storage.token_id_by_address)
    }
}
