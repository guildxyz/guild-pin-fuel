library;

use ::common::base64::base64;
use ::common::pin::PinData;
use ::interfaces::token::TokenError;

use std::call_frames::contract_id;
use std::hash::Hash;
use std::string::String;

abi PinMetadata {
    #[storage(read)]
    fn metadata(pin_id: u64) -> String;
    #[storage(read)]
    fn encoded_metadata(pin_id: u64) -> String;
}

#[storage(read)]
pub fn _metadata(pin_id: u64, key: StorageKey<StorageMap<u64, PinData>>) -> String {
    if let Some(pin_data) = key.get(pin_id).try_read() {
        pin_data.encode(pin_id)
    } else {
        require(false, TokenError::PinIdDoesNotExist);
        revert(0);
    }
}

#[storage(read)]
pub fn _encoded_metadata(pin_id: u64, key: StorageKey<StorageMap<u64, PinData>>) -> String {
    let json_metadata = _metadata(pin_id, key);
    base64(json_metadata)
}
