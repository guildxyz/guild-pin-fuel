library;

pub mod action;
pub mod base64;
pub mod claim;
pub mod pin;

use ::common::action::GuildAction;

pub type BalancesMap = StorageMap<Address, u64>;
pub type OwnersMap = StorageMap<u64, Option<Address>>;
pub type GuildIdActionTokenIdMap = StorageMap<u64, StorageMap<GuildAction, u64>>;
pub type TokenIdByAddressMap = StorageMap<Address, GuildIdActionTokenIdMap>;
pub type TokenIdByUserIdMap = StorageMap<u64, StorageKey<GuildIdActionTokenIdMap>>;
pub type TotalMintedPerGuildMap = StorageMap<u64, u64>;
