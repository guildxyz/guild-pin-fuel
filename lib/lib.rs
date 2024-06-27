#![warn(clippy::all)]
#![warn(clippy::dbg_macro)]

use fuels::types::AssetId;

// AssetId::from(0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07),
pub const ETHER_ASSET_ID: AssetId = AssetId::new([
    248, 248, 182, 40, 61, 127, 165, 182, 114, 181, 48, 203, 184, 79, 204, 203, 79, 248, 220, 64,
    248, 23, 110, 244, 84, 77, 219, 31, 25, 82, 173, 7,
]);

pub mod contract;
pub mod metadata;
pub mod parameters;
pub mod utils;
