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

        Self {
            contract,
            owner: deployer_wallet,
            user_0: user_0_wallet,
            user_1: user_1_wallet,
        }
    }

    pub async fn mint(&self, owner: Address, recipient: Address) -> FuelCallResponse<()> {
        self.contract
            .methods()
            .mint(Identity::Address(recipient), Bits256::zeroed(), 1)
            .tx_params(TxParameters::default())
            .call_params(CallParameters::default())
            .unwrap()
            .call()
            .await
            .unwrap()
    }
}
