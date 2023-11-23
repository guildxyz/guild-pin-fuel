library;

use src_20::SRC20;

use std::call_frames::contract_id;
use std::string::String;

pub fn _total_assets() -> u64 {
    1
}

#[storage(read)]
pub fn _total_supply(asset: AssetId, key: StorageKey<u64>) -> Option<u64> {
    if asset == AssetId::default(contract_id()) {
        Some(key.read())
    } else {
        None
    }
}

pub fn _name(asset: AssetId, const_name: str[9]) -> Option<String> {
    if asset == AssetId::default(contract_id()) {
        Some(String::from_ascii_str(from_str_array(const_name)))
    } else {
        None
    }
}

pub fn _symbol(asset: AssetId, const_symbol: str[5]) -> Option<String> {
    if asset == AssetId::default(contract_id()) {
        Some(String::from_ascii_str(from_str_array(const_symbol)))
    } else {
        None
    }
}

pub fn _decimals(asset: AssetId) -> Option<u8> {
    if asset == AssetId::default(contract_id()) {
        Some(0)
    } else {
        None
    }
}
