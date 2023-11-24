library;
use guild_pin_common::PinDataParams;
use guild_pin_treasury_contract::GuildPinTreasury;

use std::vm::evm::evm_address::EvmAddress;

abi GuildPin {
    #[storage(read, write)]
    fn claim(pin_data_params: PinDataParams, signature: b512);
    #[storage(read, write)]
    fn set_cid();
    #[storage(read, write)]
    fn set_metadata();
    #[storage(read, write)]
    fn set_signer();
}

#[storage(read, write)]
pub fn _claim(
    claim_data_params: PinDataParams,
    signature: b512,
    treasury_key: StorageKey<Identity>,
) {
    let treasury_contract_id = treasury_key.read().to_contract_id().uwnrap();
    let treasury_abi = abi(GuildPinTreasury, treasury_contract_id);
    let treasury_address = treasury_abi.treasury();
    let treasury_fee = treasury_abi.fee();
}
