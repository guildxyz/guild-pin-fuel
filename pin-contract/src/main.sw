contract;

mod interfaces;

use ::interfaces::init::*;

use ownership::Ownership;

use std::constants::ZERO_B256;
use std::vm::evm::evm_address::EvmAddress;

configurable {
    NAME: str[9] = __to_str_array("Guild Pin"),
    SYMBOL: str[5] = __to_str_array("GUILD"),
    OWNER: Identity = Identity::Address(Address::from(ZERO_B256)),
    SIGNER: b256 = ZERO_B256,
    TREASURY: Identity = Identity::ContractId(ContractId::from(ZERO_B256)),
    FEE: u64 = 0,
}

storage {
    owner: Ownership = Ownership::uninitialized(),
    signer: b256 = ZERO_B256,
    treasury: Identity = Identity::Address(Address::from(ZERO_B256)),
    fee: u64 = FEE,
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
