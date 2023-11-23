use fuels::prelude::*;
use fuels::programs::call_response::FuelCallResponse;
use fuels::types::Identity;

const CONTRACT_BINARY_PATH: &str = "./out/debug/guild-pin-treasury-contract.bin";
const CONTRACT_STORAGE_PATH: &str = "./out/debug/guild-pin-treasury-contract-storage_slots.json";

abigen!(Contract(
    name = "GuildTreasury",
    abi = "./treasury/out/debug/guild-pin-treasury-contract-abi.json"
));

pub struct TestContract {
    pub contract: GuildTreasury<WalletUnlocked>,
    pub owner: WalletUnlocked,
    pub account_0: WalletUnlocked,
    pub account_1: WalletUnlocked,
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
        let account_0_wallet = wallets.pop().unwrap();
        let account_1_wallet = wallets.pop().unwrap();

        // set owner at deployment
        let configurables = GuildTreasuryConfigurables::new()
            .with_OWNER(Identity::Address(Address::from(deployer_wallet.address())))
            .with_TREASURY(Identity::Address(Address::from(account_0_wallet.address())))
            .with_FEE(100);

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
            .deploy(&deployer_wallet, TxParameters::default())
            .await
            .unwrap();

        let contract = GuildTreasury::new(contract_id, contract_wallet.clone());
        let response = contract.methods().initialize().call().await.unwrap();
        let events = response
            .decode_logs_with_type::<ContractInitialized>()
            .unwrap();
        assert_eq!(
            events,
            vec![ContractInitialized {
                owner: Identity::Address(deployer_wallet.address().into()),
                treasury: Identity::Address(account_0_wallet.address().into()),
                fee: 100,
            }]
        );

        Self {
            contract,
            owner: deployer_wallet,
            account_0: account_0_wallet,
            account_1: account_1_wallet,
        }
    }

    pub async fn owner(&self) -> Address {
        let state = self.contract.methods().owner().call().await.unwrap().value;
        match state {
            State::Initialized(Identity::Address(address)) => address,
            _ => panic!("expected an initialized owner address"),
        }
    }

    pub async fn treasury(&self) -> Address {
        let treasury = self
            .contract
            .methods()
            .treasury()
            .call()
            .await
            .unwrap()
            .value;
        match treasury {
            Identity::Address(address) => address,
            _ => panic!("expected address"),
        }
    }

    pub async fn fee(&self) -> u64 {
        self.contract.methods().fee().call().await.unwrap().value
    }

    pub async fn set_owner(
        &self,
        caller: &WalletUnlocked,
        owner: Address,
    ) -> Result<FuelCallResponse<()>> {
        self.contract
            .with_account(caller.clone())
            .unwrap()
            .methods()
            .set_owner(Identity::Address(owner))
            .tx_params(TxParameters::default())
            .call_params(CallParameters::default())
            .unwrap()
            .call()
            .await
    }

    pub async fn set_treasury(
        &self,
        caller: &WalletUnlocked,
        treasury: Address,
    ) -> Result<FuelCallResponse<()>> {
        self.contract
            .with_account(caller.clone())
            .unwrap()
            .methods()
            .set_treasury(Identity::Address(treasury))
            .tx_params(TxParameters::default())
            .call_params(CallParameters::default())
            .unwrap()
            .call()
            .await
    }

    pub async fn set_fee(&self, caller: &WalletUnlocked, fee: u64) -> Result<FuelCallResponse<()>> {
        self.contract
            .with_account(caller.clone())
            .unwrap()
            .methods()
            .set_fee(fee)
            .tx_params(TxParameters::default())
            .call_params(CallParameters::default())
            .unwrap()
            .call()
            .await
    }
}
