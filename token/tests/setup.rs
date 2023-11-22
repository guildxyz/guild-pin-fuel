use fuels::prelude::*;
use fuels::programs::call_response::FuelCallResponse;
use fuels::types::{Bits256, Identity};

const CONTRACT_BINARY_PATH: &str = "./out/debug/guild-pin-token-contract.bin";
const CONTRACT_STORAGE_PATH: &str = "./out/debug/guild-pin-token-contract-storage_slots.json";

abigen!(Contract(
    name = "GuildToken",
    abi = "./token/out/debug/guild-pin-token-contract-abi.json"
));

pub struct TestContract {
    pub contract: GuildToken<WalletUnlocked>,
    pub owner: WalletUnlocked,
    pub user_0: WalletUnlocked,
    pub user_1: WalletUnlocked,
}

impl TestContract {
    pub async fn new() -> Self {
        // configure wallets
        let number_of_wallets = 4;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000_000;
        let wallet_config = WalletsConfig::new(
            Some(number_of_wallets),
            Some(coins_per_wallet),
            Some(amount_per_coin),
        );
        let mut wallets = launch_custom_provider_and_get_wallets(wallet_config, None, None)
            .await
            .unwrap();

        let deployer_wallet = wallets.pop().unwrap();
        let contract_wallet = wallets.pop().unwrap();
        let user_0_wallet = wallets.pop().unwrap();
        let user_1_wallet = wallets.pop().unwrap();

        // set owner at deployment
        let configurables = GuildTokenConfigurables::new()
            .with_OWNER(Identity::Address(Address::from(deployer_wallet.address())));

        // load storage configuration
        let storage_configuration = StorageConfiguration::default()
            .add_slot_overrides_from_file(CONTRACT_STORAGE_PATH)
            .unwrap();

        // deploy contract
        let configuration = LoadConfiguration::default()
            .with_storage_configuration(storage_configuration)
            .with_configurables(configurables);
        let guild_token_contract_id = Contract::load_from(CONTRACT_BINARY_PATH, configuration)
            .unwrap()
            .deploy(&deployer_wallet, TxParameters::default())
            .await
            .unwrap();

        let contract = GuildToken::new(guild_token_contract_id, contract_wallet.clone());
        contract.methods().initialize().call().await.unwrap();

        Self {
            contract,
            owner: deployer_wallet,
            user_0: user_0_wallet,
            user_1: user_1_wallet,
        }
    }

    pub async fn owner(&self) -> Address {
        let state = self.contract.methods().owner().call().await.unwrap().value;
        match state {
            State::Initialized(Identity::Address(address)) => address,
            _ => panic!("expected an initialized owner address"),
        }
    }

    pub async fn mint(
        &self,
        caller: &WalletUnlocked,
        recipient: Address,
    ) -> Result<FuelCallResponse<()>> {
        self.unsafe_mint(caller, recipient, Bits256::zeroed(), 1)
            .await
    }

    pub async fn unsafe_mint(
        &self,
        caller: &WalletUnlocked,
        recipient: Address,
        sub_id: Bits256,
        amount: u64,
    ) -> Result<FuelCallResponse<()>> {
        let asset_id = self.contract.contract_id().asset_id(&sub_id);
        self.contract
            .with_account(caller.clone())
            .unwrap()
            .methods()
            .mint(Identity::Address(recipient), sub_id, amount)
            .tx_params(TxParameters::default())
            .call_params(CallParameters::default().with_asset_id(asset_id))
            .unwrap()
            .call()
            .await
    }

    pub async fn burn(&self, caller: &WalletUnlocked, pin_id: u64) -> Result<FuelCallResponse<()>> {
        self.unsafe_burn(caller, Bits256::zeroed(), pin_id).await
    }

    pub async fn unsafe_burn(
        &self,
        caller: &WalletUnlocked,
        sub_id: Bits256,
        pin_id: u64,
    ) -> Result<FuelCallResponse<()>> {
        let asset_id = self.contract.contract_id().asset_id(&sub_id);
        self.contract
            .with_account(caller.clone())
            .unwrap()
            .methods()
            .burn(sub_id, pin_id)
            .tx_params(TxParameters::default())
            .call_params(CallParameters::default().with_asset_id(asset_id))
            .unwrap()
            .call()
            .await
    }

    pub async fn set_owner(
        &self,
        caller: &WalletUnlocked,
        new_owner: Address,
    ) -> Result<FuelCallResponse<()>> {
        self.contract
            .with_account(caller.clone())
            .unwrap()
            .methods()
            .set_owner(Identity::Address(new_owner))
            .tx_params(TxParameters::default())
            .call_params(CallParameters::default())
            .unwrap()
            .call()
            .await
    }

    pub async fn balance(&self, of: Address) -> FuelCallResponse<u64> {
        self.contract
            .methods()
            .balance(Identity::Address(of))
            .call()
            .await
            .unwrap()
    }

    pub async fn pin_owner(&self, pin_id: u64) -> FuelCallResponse<Option<Identity>> {
        self.contract
            .methods()
            .pin_owner(pin_id)
            .call()
            .await
            .unwrap()
    }

    pub async fn total_supply(&self) -> u64 {
        let asset_id = self.contract.contract_id().asset_id(&Bits256::zeroed());
        self.contract
            .methods()
            .total_supply(asset_id)
            .call()
            .await
            .unwrap()
            .value
            .unwrap()
    }

    pub async fn total_minted(&self) -> u64 {
        self.contract
            .methods()
            .total_minted()
            .call()
            .await
            .unwrap()
            .value
    }
}
