use crate::parameters::Parameters;

use fuels::prelude::*;
use fuels::programs::call_response::FuelCallResponse;
use fuels::types::{Bits256, Identity};

const CONTRACT_BINARY_PATH: &str = "./out/debug/guild-pin-contract.bin";
const CONTRACT_STORAGE_PATH: &str = "./out/debug/guild-pin-contract-storage_slots.json";

abigen!(Contract(
    name = "GuildPin",
    abi = "./pin-contract/out/debug/guild-pin-contract-abi.json"
));

pub struct GuildPinContract(GuildPin<WalletUnlocked>);

impl GuildPinContract {
    pub async fn new(parameters: Parameters) -> Self {
        todo!()
    }
}
