library;

pub mod utils;
pub mod action;
pub mod base64;
pub mod claim;
pub mod pin;

use ::common::action::GuildAction;

use std::call_frames::get_contract_id_from_call_frame;
use std::registers::frame_ptr;

pub type BalancesMap = StorageMap<Address, u64>;
pub type OwnersMap = StorageMap<u64, Option<Address>>;
pub type GuildIdActionTokenIdMap = StorageMap<u64, StorageMap<GuildAction, u64>>;
pub type TokenIdByAddressMap = StorageMap<Address, GuildIdActionTokenIdMap>;
pub type TokenIdByUserIdMap = StorageMap<u64, StorageKey<GuildIdActionTokenIdMap>>;
pub type TotalMintedPerGuildMap = StorageMap<u64, u64>;
pub type TokenOfOwnerByIndexMap = StorageMap<Address, StorageMap<u64, u64>>;

pub fn contract_id() -> ContractId {
    let current_call_frame = frame_ptr();
    get_contract_id_from_call_frame(current_call_frame)
}
