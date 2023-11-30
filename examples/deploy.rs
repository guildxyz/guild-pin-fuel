use fuels::accounts::fuel_crypto::SecretKey;
use fuels::accounts::provider::Provider;
use fuels::accounts::wallet::Wallet;
use fuels::prelude::{abigen, TxParameters};
use fuels::programs::contract::{Contract, LoadConfiguration, StorageConfiguration};
use fuels::types::bech32::Bech32Address;
use fuels::types::{Address, AssetId, Bits256, EvmAddress, Identity};
use signrs::eth::EthSigner;

use std::io::Read;
use std::str::FromStr;

const TESTNET_URL: &str = "https://beta-4.fuel.network/";
const DEPLOYER: &str = "fuel1ssfz00nnwm50vharhn3t9guhet4sp30qsxkyyvyzxqhep50va4rsxalmsg";
const DEPLOYER_ADDRESS: &str = "841227be7376e8f65fa3bce2b2a397caeb00c5e081ac423082302f90d1eced47";
const TREASURY: &str = "fuel1lt9ajl4tn3ne9hlrpc4jggeljrrsdns565twvmy5aycytedgmclsc6f6qy";
const TREASURY_ADDRESS: &str = "facbd97eab9c6792dfe30e2b24233f90c706ce14d516e66c94e93045e5a8de3f";

const CONTRACT_BINARY_PATH: &str = "./out/debug/guild-pin-contract.bin";
const CONTRACT_STORAGE_PATH: &str = "./out/debug/guild-pin-contract-storage_slots.json";

abigen!(Contract(
    name = "GuildPin",
    abi = "./out/debug/guild-pin-contract-abi.json"
));

#[tokio::main]
async fn main() {
    let secret_key_string = std::fs::read_to_string("../wallets/fuel-tn-deployer-sk").unwrap();
    let secret_key = SecretKey::from_str(&secret_key_string).unwrap();
    let provider = Provider::connect(TESTNET_URL).await.unwrap();
    let deployer_address = Bech32Address::from(Address::from_str(DEPLOYER_ADDRESS).unwrap());
    let treasury_address = Address::from_str(TREASURY_ADDRESS).unwrap();
    let deployer = Wallet::from_address(deployer_address, Some(provider)).unlock(secret_key);

    println!(
        "{:?}",
        deployer
            .provider()
            .unwrap()
            .get_coins(deployer.address(), AssetId::BASE)
            .await
            .unwrap()
    );

    let signer_seed_string = std::fs::read_to_string("../wallets/fuel-signer-seed").unwrap();
    let signer_seed: [u8; 32] = serde_json::from_str(&signer_seed_string).unwrap();
    let signer = EthSigner::new(&signer_seed);
    let mut bytes = [0u8; 32];
    bytes[12..].copy_from_slice(&signer.address());
    let signer_address = EvmAddress::from(Bits256(bytes));
    println!("{:?}", signer_address);

    let configurables = GuildPinConfigurables::new()
        .with_OWNER(Identity::Address(Address::from(deployer.address())))
        .with_TREASURY(Identity::Address(treasury_address))
        .with_SIGNER(signer_address.value())
        .with_FEE(10);

    // load storage configuration
    let storage_configuration = StorageConfiguration::default()
        .add_slot_overrides_from_file(CONTRACT_STORAGE_PATH)
        .unwrap();

    // deploy contract
    let configuration = LoadConfiguration::default()
        .with_storage_configuration(storage_configuration)
        .with_configurables(configurables);
    
    //let contract_id = Contract::load_from(CONTRACT_BINARY_PATH, configuration)
    //    .unwrap()
    //    .deploy(&deployer, TxParameters::default())
    //    .await
    //    .unwrap();

    println!("CONTRACT_ID = {:?}", contract_id);

    let contract = GuildPin::new(contract_id, deployer.clone());

    let response = contract.with_account(deployer.clone()).unwrap().methods().initialize().call().await.unwrap();
    println!("{:?}", response);
    let owner = contract.methods().owner().call().await.unwrap();
    println!("OWNER: {:?}", owner);
}
