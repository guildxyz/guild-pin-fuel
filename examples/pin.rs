use base64::{engine::general_purpose::STANDARD, Engine as _};
use fuels::accounts::fuel_crypto::fuel_types::Salt;
use fuels::prelude::{Address, AssetId};
use fuels::types::EvmAddress;
use guild_pin_contract::contract::{GuildAction, GuildPinContract};
use guild_pin_contract::metadata::TokenUri;
use guild_pin_contract::parameters::ParametersBuilder;
use guild_pin_contract::utils::{bytes_to_b256, ClaimBuilder};
use structopt::StructOpt;

use std::path::PathBuf;

#[derive(StructOpt, Debug)]
struct Pin {
    #[structopt(default_value = "https://beta-5.fuel.network/")]
    url: String,
    #[structopt(default_value = "../wallets/fuel-signer-seed")]
    signer: PathBuf,
    #[structopt(default_value = "../wallets/fuel-tn-deployer-sk")]
    deployer: PathBuf,
    #[structopt(default_value = "../wallets/fuel-tn-treasury-sk")]
    treasury: PathBuf,
    #[structopt(default_value = "1")]
    version: u8,
    #[structopt(subcommand)]
    contract: Option<Contract>,
}

#[derive(StructOpt, Debug)]
enum Contract {
    Deploy,
    SetSigner {
        // default is the guild backend signer address
        #[structopt(default_value = "0x989a6C5D84c932E7c9EaE8b4D2d5f378b11C21F7")]
        signer: String,
    },
    SetFee {
        #[structopt(default_value = "15")]
        fee: u32,
    },
    TestClaim,
}


#[tokio::main]
async fn main() {
    let pin = Pin::from_args();

    println!("pin: {:#?}", pin);

    let parameters = ParametersBuilder::new()
        .signer_file(pin.signer)
        .owner_file(pin.deployer)
        .treasury_file(pin.treasury)
        .url(&pin.url)
        .salt(Salt::new([pin.version; 32]))
        .build()
        .await;

    // print local data
    println!("LOCAL DATA");
    println!("owner: {}", Address::from(parameters.owner.address()));
    println!("treasury: {}", Address::from(parameters.treasury.address()));
    println!("signer: 0x{}", hex::encode(parameters.signer.address()));

    /*
    let contract = if std::env::var("DEPLOY").is_ok() {
        GuildPinContract::init(&parameters).await
    } else {
        GuildPinContract::new(&parameters)
    };

    println!("contract id: {}", contract.contract_id());

    // print on-chain queries
    println!("ON-CHAIN QUERIES");
    let balance = parameters
        .provider()
        .get_asset_balance(parameters.owner.address(), AssetId::BASE)
        .await
        .unwrap();
    println!("balance: {}", balance);
    println!("owner: {:?}", contract.owner().await.unwrap());
    println!("treasury: {:?}", contract.treasury().await.unwrap());
    println!("fee: {:?}", contract.fee().await.unwrap());
    println!(
        "signer: 0x{}",
        hex::encode(&contract.signer().await.unwrap().value().0[12..])
    );

    // set signer
    if std::env::var("SET_SIGNER").is_ok() {
        let signer_bytes = hex::decode(BACKEND_SIGNER.trim_start_matches("0x")).unwrap();
        let new_signer = EvmAddress::from(bytes_to_b256(&signer_bytes));
        contract
            .set_signer(&parameters.owner, new_signer)
            .await
            .unwrap();
        println!(
            "new signer: 0x{:?}",
            hex::encode(&contract.signer().await.unwrap().value().0[12..])
        );
    }

    // set fee
    if std::env::var("SET_FEE").is_ok() {
        contract.set_fee(&parameters.owner, 15).await.unwrap();
        println!("new fee: {:?}", contract.fee().await.unwrap());
    }
    let user_id = u64::MAX;
    let guild_id = u64::MAX;
    let action = GuildAction::Owner;
    // send claim
    if std::env::var("CLAIM").is_ok() {
        let claim = ClaimBuilder::new(parameters.owner.address().into(), contract.contract_id())
            .action(action)
            .user_id(user_id)
            .guild_id(guild_id)
            .build();
        let signature = parameters.sign_claim(&claim);
        contract
            .claim(&parameters.owner, claim, signature)
            .await
            .unwrap();
    }

    let mut header = contract.encoded_metadata(0).await.unwrap();
    let encoded_metadata = header.split_off(29);
    assert_eq!(header, "data:application/json;base64,");
    let decoded_metadata = String::from_utf8(STANDARD.decode(encoded_metadata).unwrap()).unwrap();
    let token_uri: TokenUri = serde_json::from_str(&decoded_metadata).unwrap();
    println!("token uri: {:#?}", token_uri);

    let balance = parameters
        .provider()
        .get_asset_balance(parameters.owner.address(), AssetId::BASE)
        .await
        .unwrap();
    println!("balance: {}", balance);
    */
}
