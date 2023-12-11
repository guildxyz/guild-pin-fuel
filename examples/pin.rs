use base64::{engine::general_purpose::STANDARD, Engine as _};
use fuels::accounts::fuel_crypto::fuel_types::Salt;
use fuels::prelude::Address;
use fuels::types::EvmAddress;
use guild_pin_contract::contract::{GuildAction, GuildPinContract};
use guild_pin_contract::metadata::TokenUri;
use guild_pin_contract::parameters::ParametersBuilder;
use guild_pin_contract::utils::{bytes_to_b256, ClaimBuilder};

const TESTNET_URL: &str = "https://beta-4.fuel.network/";

const BACKEND_SIGNER: &str = "0x989a6C5D84c932E7c9EaE8b4D2d5f378b11C21F7";

#[tokio::main]
async fn main() {
    let parameters = ParametersBuilder::new()
        .signer_file("../wallets/fuel-signer-seed")
        .owner_file("../wallets/fuel-tn-deployer-sk")
        .treasury_file("../wallets/fuel-tn-treasury-sk")
        .url(TESTNET_URL)
        .salt(Salt::new([1u8; 32]))
        .build()
        .await;

    println!("OWNER: {}", Address::from(parameters.owner.address()));
    println!("TREASURY: {}", Address::from(parameters.treasury.address()));
    println!("SIGNER: 0x{}", hex::encode(parameters.signer.address()));

    let contract = if std::env::var("DEPLOY").is_ok() {
        GuildPinContract::init(&parameters).await
    } else {
        GuildPinContract::new(&parameters)
    };

    println!("CONTRACT ID: {}", contract.contract_id());

    println!("OWNER QUERY: {:?}", contract.owner().await.unwrap());
    println!("TREASURY QUERY: {:?}", contract.treasury().await.unwrap());

    println!("FEE QUERY: {:?}", contract.fee().await.unwrap());

    println!(
        "SIGNER QUERY: 0x{}",
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
    }

    // set fee
    if std::env::var("SET_FEE").is_ok() {
        contract.set_fee(&parameters.owner, 15).await.unwrap();
        println!("FEE QUERY: {:?}", contract.fee().await.unwrap());
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
    println!("TOKEN URI: {:#?}", token_uri);
}
