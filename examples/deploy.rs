use fuels::accounts::provider::Provider;
use fuels::types::bech32::Bech32Address;
use fuels::types::{Address, AssetId, Bits256, EvmAddress};
use signrs::eth::EthSigner;

use std::fs::File;
use std::io::Read;
use std::str::FromStr;

const TESTNET_URL: &str = "https://beta-4.fuel.network/";
const DEPLOYER: &str = "fuel1ssfz00nnwm50vharhn3t9guhet4sp30qsxkyyvyzxqhep50va4rsxalmsg";
const DEPLOYER_ADDRESS: &str = "841227be7376e8f65fa3bce2b2a397caeb00c5e081ac423082302f90d1eced47";
const TREASURY: &str = "fuel1lt9ajl4tn3ne9hlrpc4jggeljrrsdns565twvmy5aycytedgmclsc6f6qy";
const TREASURY_ADDRESS: &str = "facbd97eab9c6792dfe30e2b24233f90c706ce14d516e66c94e93045e5a8de3f";

#[tokio::main]
async fn main() {
    let mut file = File::open("/data/zgen/wallets/fuel-signer-seeed").unwrap();
    let mut signer_seed = Vec::with_capacity(32);
    file.read_to_end(&mut signer_seed).unwrap();
    let signer = EthSigner::new(&signer_seed);
    let mut bytes = [0u8; 32];
    bytes[12..].copy_from_slice(&signer.address());
    let signer_address = EvmAddress::from(Bits256(bytes));
    println!("{:?}", signer_address);
    let provider = Provider::connect(TESTNET_URL).await.unwrap();
    let deployer = Bech32Address::from(Address::from_str(DEPLOYER_ADDRESS).unwrap());
    println!(
        "{:?}",
        provider.get_coins(&deployer, AssetId::BASE).await.unwrap()
    );
}
