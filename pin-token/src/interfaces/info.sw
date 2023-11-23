library;

use ::interfaces::src3::TokenError;

use std::hash::Hash;

abi Info {
    #[storage(read)]
    fn balance(of: Identity) -> u64;
    #[storage(read)]
    fn pin_owner(pin_id: u64) -> Option<Identity>;
    #[storage(read)]
    fn total_minted() -> u64;
}

#[storage(read)]
pub fn _balance(
    of: Identity,
    balances_key: StorageKey<StorageMap<Identity, u64>>,
) -> u64 {
    balances_key.get(of).try_read().unwrap_or(0)
}

#[storage(read)]
pub fn _pin_owner(
    pin_id: u64,
    owners_key: StorageKey<StorageMap<u64, Option<Identity>>>,
) -> Option<Identity> {
    let maybe_owner = owners_key.get(pin_id).try_read();
    if let Some(owner) = maybe_owner {
        owner
    } else {
        require(false, TokenError::PinIdDoesNotExist);
        revert(155)
    }
}
