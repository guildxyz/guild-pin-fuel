library;

pub struct ContractInitialized {
    owner: Identity,
    treasury: Identity,
    fee: u64,
}

pub struct OwnerSet {
    old: Identity,
    new: Identity,
}

pub struct FeeSet {
    old: u64,
    new: u64,
}

pub struct TreasurySet {
    old: Identity,
    new: Identity,
}
