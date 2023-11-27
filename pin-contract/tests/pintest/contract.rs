use crate::parameters::Parameters;

use fuels::prelude::*;
use fuels::programs::call_response::FuelCallResponse;
use fuels::types::errors::Error;
use fuels::types::{Bits256, Identity};

const CONTRACT_BINARY_PATH: &str = "./out/debug/guild-pin-contract.bin";
const CONTRACT_STORAGE_PATH: &str = "./out/debug/guild-pin-contract-storage_slots.json";

abigen!(Contract(
    name = "GuildPin",
    abi = "./pin-contract/out/debug/guild-pin-contract-abi.json"
));

pub struct GuildPinContract(GuildPin<WalletUnlocked>);

impl GuildPinContract {
    pub async fn new(parameters: &Parameters) -> Self {
        // initialize configurables
        let configurables = GuildPinConfigurables::new()
            .with_OWNER(Identity::Address(Address::from(parameters.owner.address())))
            .with_TREASURY(Identity::Address(Address::from(
                parameters.treasury.address(),
            )))
            .with_SIGNER(parameters.signer_b256())
            .with_FEE(parameters.fee);

        // load storage configuration
        let storage_configuration = StorageConfiguration::default()
            .add_slot_overrides_from_file(CONTRACT_STORAGE_PATH)
            .unwrap();

        // deploy contract
        let configuration = LoadConfiguration::default()
            .with_storage_configuration(storage_configuration)
            .with_configurables(configurables);
        let contract_id = Contract::load_from(CONTRACT_BINARY_PATH, configuration)
            .unwrap()
            .deploy(&parameters.owner, TxParameters::default())
            .await
            .unwrap();

        Self(GuildPin::new(contract_id, parameters.contract.clone()))
    }

    pub async fn init(parameters: &Parameters) -> Self {
        let contract = Self::new(parameters).await;
        contract.initialize(&parameters.owner).await.unwrap();
        contract
    }

    pub async fn initialize(&self, caller: &WalletUnlocked) -> Result<FuelCallResponse<()>> {
        self.0
            .with_account(caller.clone())
            .unwrap()
            .methods()
            .initialize()
            .call()
            .await
    }

    pub async fn set_owner(
        &self,
        caller: &WalletUnlocked,
        owner: Identity,
    ) -> Result<FuelCallResponse<()>> {
        self.0
            .with_account(caller.clone())
            .unwrap()
            .methods()
            .set_owner(owner)
            .call()
            .await
    }

    pub async fn owner(&self) -> Result<Identity> {
        let state = self.0.methods().owner().call().await?.value;
        match state {
            State::Initialized(owner) => Ok(owner),
            _ => Err(Error::InvalidData("NotInitialized".to_string())),
        }
    }
}
