use base64::{engine::general_purpose::STANDARD, Engine as _};
use fuels::accounts::provider::Provider;
use fuels::prelude::Salt;
use fuels::types::bech32::Bech32Address;
use fuels::types::{Address, EvmAddress, Identity};
use guild_pin_contract::contract::{GuildAction, GuildPinContract};
use guild_pin_contract::metadata::TokenUri;
use guild_pin_contract::parameters::Parameters;
use guild_pin_contract::parameters::ParametersBuilder;
use guild_pin_contract::utils::{bytes_to_b256, ClaimBuilder};
use guild_pin_contract::ETHER_ASSET_ID;
use signrs::Signer;
use structopt::StructOpt;

use std::path::PathBuf;
use std::str::FromStr;

#[derive(StructOpt, Debug)]
struct Pin {
    /// The URL of the Fuel chain.
    ///
    /// The default value is the testnet URL.
    #[structopt(default_value = "https://testnet.fuel.network/")]
    url: String,
    /// Path to the Guild Pin backend signer's secret key file.
    ///
    /// A Guild-authorized signature is required by the contract to mint pins for users. This
    /// signer is already initialized on the Guild backend, however, it can be overridden for
    /// testing purposes.
    #[structopt(default_value = "../wallets/fuel-signer-seed")]
    signer: PathBuf,
    /// Path to the contract deployer's secret key file.
    ///
    /// The deployer will be the admin of the contract who can dispatch admin level contract calls,
    /// such as setting the backend signer address, the treasury or the minting fee.
    #[structopt(default_value = "../wallets/fuel-tn-deployer-sk")]
    deployer: PathBuf,
    /// Path to the treasury's secret key file.
    ///
    /// This will be the address where minting fees are accumulated.
    #[structopt(default_value = "../wallets/fuel-tn-treasury-sk")]
    treasury: PathBuf,
    /// Current deployment version of the contract.
    ///
    /// If you re-deploy the contract with a bumped version, it will start with a new state at a
    /// new address that's derived from a salt generated from the version number.
    #[structopt(default_value = "1")]
    version: u8,
    /// Optional contract interaction command.
    #[structopt(subcommand)]
    contract: Option<Contract>,
}

#[derive(StructOpt, Debug)]
enum Contract {
    /// Deploys the contract with the given parameters.
    Deploy,
    /// An admin-level contract call that sets the backend signer's EVM address.
    ///
    /// Pins will be minted only if a valid signature is submitted from this address. The default
    /// value is the Guild backend signer address.
    SetSigner {
        #[structopt(default_value = "0x989a6C5D84c932E7c9EaE8b4D2d5f378b11C21F7")]
        signer: String,
    },
    /// An admin-level contract call that sets minting fee collected by the treasury.
    SetFee {
        #[structopt(default_value = "15")]
        fee: u64,
    },
    /// An admin-level contract call that sets the treasury address collecting minting fees.
    SetTreasury { treasury: String },
    /// Dispatches a test claim for minting a Guild pin.
    ///
    /// This only works if the backend signer is set to the test signer. Otherwise the internally
    /// generated signature will be invalid against the actual backend signer's address.
    TestClaim {
        #[structopt(short = "u", long)]
        user_id: u64,
        #[structopt(short = "g", long)]
        guild_id: u64,
        #[structopt(default_value = "owner")]
        action: String,
    },
    /// Fetches the metadata of a given pin.
    ///
    /// If the pin id is not submitted, it attempts to read the last minted pin's metadata.
    Metadata {
        #[structopt(short = "p", long)]
        pin_id: Option<u64>,
    },
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

    // print addresses and balances
    print_balances(&parameters).await;

    // deploy/initialize contract
    let contract = if let Some(Contract::Deploy) = pin.contract {
        GuildPinContract::init(&parameters).await
    } else {
        GuildPinContract::new(&parameters)
    };

    println!("contract id: {}", contract.contract_id());
    query_storage(&contract).await;

    // interact with the contract
    match pin.contract {
        Some(Contract::SetSigner { signer }) => set_signer(&parameters, &contract, signer).await,
        Some(Contract::SetFee { fee }) => set_fee(&parameters, &contract, fee).await,
        Some(Contract::SetTreasury { treasury }) => {
            set_treasury(&parameters, &contract, &treasury).await
        }
        Some(Contract::TestClaim {
            user_id,
            guild_id,
            action,
        }) => {
            test_claim(&parameters, &contract, user_id, guild_id, action).await;
            read_last_metadata(&contract).await;
        }
        Some(Contract::Metadata { pin_id }) => {
            if let Some(id) = pin_id {
                read_metadata(&contract, id).await;
            } else {
                read_last_metadata(&contract).await;
            }
        }
        _ => unimplemented!(),
    }

    print_balances(&parameters).await;
    query_storage(&contract).await;
}

async fn eth_balance(provider: &Provider, address: &Bech32Address) -> u64 {
    provider
        .get_asset_balance(address, ETHER_ASSET_ID)
        .await
        .unwrap()
}

async fn signer_in_storage(contract: &GuildPinContract) -> String {
    let signer_in_storage = &contract.signer().await.unwrap().value().0[12..];
    hex::encode(signer_in_storage)
}

async fn print_balances(parameters: &Parameters) {
    println!("ADDRESSES WITH BALANCES");
    let owner = parameters.owner.address();
    let treasury = parameters.treasury.address();
    let owner_balance = eth_balance(parameters.provider(), parameters.owner.address()).await;
    let treasury_balance = eth_balance(parameters.provider(), parameters.treasury.address()).await;
    println!("owner:    {}\t balance: {}", owner, owner_balance);
    println!("treasury: {}\t balance: {}", treasury, treasury_balance);
    println!("signer:   0x{}", hex::encode(parameters.signer.address()));
}

async fn query_storage(contract: &GuildPinContract) {
    println!("ON-CHAIN QUERIES");
    println!("owner:    {:?}", contract.owner().await.unwrap());
    println!("treasury: {:?}", contract.treasury().await.unwrap());
    println!("fee:      {}", contract.fee().await.unwrap());
    println!("signer:   0x{}", signer_in_storage(contract).await);
}

async fn set_signer(parameters: &Parameters, contract: &GuildPinContract, hex_signer: String) {
    let signer_bytes = hex::decode(hex_signer.trim_start_matches("0x")).unwrap();
    let new_signer = EvmAddress::from(bytes_to_b256(&signer_bytes));
    contract
        .set_signer(&parameters.owner, new_signer)
        .await
        .unwrap();
    println!("signer set successfully");
    println!("new signer: 0x{:?}", signer_in_storage(contract).await);
}

async fn set_fee(parameters: &Parameters, contract: &GuildPinContract, fee: u64) {
    contract.set_fee(&parameters.owner, fee).await.unwrap();
    println!("new fee: {:?}", contract.fee().await.unwrap());
}

async fn set_treasury(parameters: &Parameters, contract: &GuildPinContract, treasury: &str) {
    let treasury = Address::from_str(treasury).expect("invalid treasury address");
    contract
        .set_treasury(&parameters.owner, Identity::from(treasury))
        .await
        .unwrap();
    println!("new treasury: {:?}", contract.treasury().await.unwrap());
}

async fn test_claim(
    parameters: &Parameters,
    contract: &GuildPinContract,
    user_id: u64,
    guild_id: u64,
    action: String,
) {
    let action = match action.as_str() {
        "joined" => GuildAction::Joined,
        "owner" => GuildAction::Owner,
        "admin" => GuildAction::Admin,
        s => panic!("invalid action {}", s),
    };
    // send claim
    let claim = ClaimBuilder::new(parameters.owner.address().into(), contract.contract_id())
        .action(action)
        .user_id(user_id)
        .guild_id(guild_id)
        .build();
    let signature = parameters.sign_claim(&claim);
    contract
        .claim_eth(&parameters.owner, claim, signature)
        .await
        .unwrap();
}

async fn read_metadata(contract: &GuildPinContract, pin_id: u64) {
    let mut header = contract.encoded_metadata(pin_id).await.unwrap();
    let encoded_metadata = header.split_off(29);
    assert_eq!(header, "data:application/json;base64,");
    let decoded_metadata = String::from_utf8(STANDARD.decode(encoded_metadata).unwrap()).unwrap();
    let token_uri: TokenUri = serde_json::from_str(&decoded_metadata).unwrap();
    println!("token uri: {:#?}", token_uri);
}

async fn read_last_metadata(contract: &GuildPinContract) {
    let last_pin_id = contract.total_minted().await.unwrap().saturating_sub(1);
    read_metadata(contract, last_pin_id).await;
}
