use crate::parameters::Parameters;

use fuels::prelude::*;
use fuels::programs::call_response::FuelCallResponse;
use fuels::programs::call_utils::TxDependencyExtension;
use fuels::programs::contract::CallParameters;
use fuels::types::errors::Error;
use fuels::types::{AssetId, Bits256, ContractId, EvmAddress, Identity, B512};

const CONTRACT_BINARY_PATH: &str = "./out/debug/guild-pin-contract.bin";
const CONTRACT_STORAGE_PATH: &str = "./out/debug/guild-pin-contract-storage_slots.json";

abigen!(Contract(
    name = "GuildPin",
    abi = "./out/debug/guild-pin-contract-abi.json"
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

    pub fn bech_contract_id(&self) -> &Bech32ContractId {
        self.0.contract_id()
    }

    pub fn contract_id(&self) -> ContractId {
        self.bech_contract_id().into()
    }

    pub fn asset_id(&self) -> AssetId {
        self.bech_contract_id().asset_id(&Bits256::zeroed())
    }

    pub async fn initialize(&self, caller: &WalletUnlocked) -> Result<FuelCallResponse<()>> {
        self.0
            .with_account(caller.clone())?
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
            .with_account(caller.clone())?
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

    pub async fn set_signer(
        &self,
        caller: &WalletUnlocked,
        signer: EvmAddress,
    ) -> Result<FuelCallResponse<()>> {
        self.0
            .with_account(caller.clone())?
            .methods()
            .set_signer(signer)
            .call()
            .await
    }

    pub async fn signer(&self) -> Result<EvmAddress> {
        let inner = self.0.methods().signer().call().await?.value;
        Ok(EvmAddress::from(inner))
    }

    pub async fn set_treasury(
        &self,
        caller: &WalletUnlocked,
        treasury: Identity,
    ) -> Result<FuelCallResponse<()>> {
        self.0
            .with_account(caller.clone())?
            .methods()
            .set_treasury(treasury)
            .call()
            .await
    }

    pub async fn treasury(&self) -> Result<Identity> {
        Ok(self.0.methods().treasury().call().await?.value)
    }

    pub async fn set_fee(&self, caller: &WalletUnlocked, fee: u64) -> Result<FuelCallResponse<()>> {
        self.0
            .with_account(caller.clone())?
            .methods()
            .set_fee(fee)
            .call()
            .await
    }

    pub async fn fee(&self) -> Result<u64> {
        Ok(self.0.methods().fee().call().await?.value)
    }

    pub async fn claim(
        &self,
        caller: &WalletUnlocked,
        params: ClaimParameters,
        signature: B512,
    ) -> Result<FuelCallResponse<()>> {
        let total_fee = self.fee().await? + params.admin_fee;
        let asset_id = AssetId::BASE;
        self.unsafe_claim(caller, params, signature, total_fee, asset_id)
            .await
    }

    pub async fn unsafe_claim(
        &self,
        caller: &WalletUnlocked,
        params: ClaimParameters,
        signature: B512,
        total_fee: u64,
        asset_id: AssetId,
    ) -> Result<FuelCallResponse<()>> {
        self.0
            .with_account(caller.clone())?
            .methods()
            .claim(params, signature)
            .append_variable_outputs(2) // needed for sending tokens to optionally 2 treasuries
            .call_params(
                CallParameters::default()
                    .with_asset_id(asset_id)
                    .with_amount(total_fee),
            )?
            .call()
            .await
    }

    pub async fn balance_of(&self, id: Address) -> Result<u64> {
        self.0
            .methods()
            .balance_of(id)
            .call()
            .await
            .map(|r| r.value)
    }

    pub async fn pin_owner(&self, pin_id: u64) -> Result<Option<Address>> {
        self.0
            .methods()
            .pin_owner(pin_id)
            .call()
            .await
            .map(|r| r.value)
    }

    pub async fn total_minted(&self) -> Result<u64> {
        self.0
            .methods()
            .total_minted()
            .call()
            .await
            .map(|r| r.value)
    }

    pub async fn total_minted_per_guild(&self, guild_id: u64) -> Result<u64> {
        self.0
            .methods()
            .total_minted_per_guild(guild_id)
            .call()
            .await
            .map(|r| r.value)
    }

    pub async fn pin_id_by_address(
        &self,
        address: Address,
        guild_id: u64,
        action: GuildAction,
    ) -> Result<Option<u64>> {
        self.0
            .methods()
            .pin_id_by_address(address, guild_id, action)
            .call()
            .await
            .map(|r| r.value)
    }

    pub async fn pin_id_by_user_id(
        &self,
        user_id: u64,
        guild_id: u64,
        action: GuildAction,
    ) -> Result<Option<u64>> {
        self.0
            .methods()
            .pin_id_by_user_id(user_id, guild_id, action)
            .call()
            .await
            .map(|r| r.value)
    }

    pub async fn metadata(&self, pin_id: u64) -> Result<String> {
        self.0
            .methods()
            .metadata(pin_id)
            .call()
            .await
            .map(|r| r.value)
    }

    pub async fn encoded_metadata(&self, pin_id: u64) -> Result<String> {
        self.0
            .methods()
            .encoded_metadata(pin_id)
            .call()
            .await
            .map(|r| r.value)
    }
}
