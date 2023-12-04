use fuels::prelude::Address;
use guild_pin_contract::contract::GuildPinContract;
use guild_pin_contract::parameters::ParametersBuilder;

const TESTNET_URL: &str = "https://beta-4.fuel.network/";

#[tokio::main]
async fn main() {
    let parameters = ParametersBuilder::new()
        .signer_file("../wallets/fuel-signer-seed")
        .owner_file("../wallets/fuel-tn-deployer-sk")
        .treasury_file("../wallets/fuel-tn-treasury-sk")
        .url(TESTNET_URL)
        .build()
        .await;

    println!("OWNER: {}", Address::from(parameters.owner.address()));
    println!("TREASURY: {}", Address::from(parameters.treasury.address()));

    let contract = GuildPinContract::new(&parameters);

    println!(
        "OWNER QUERY FROM CHAIN: {:?}",
        contract.owner().await.unwrap()
    );
}
