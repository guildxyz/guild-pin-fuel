library;

pub struct ContractInitialized {
    owner: Identity,
}

pub struct PinMinted {
    recipient: Identity,
    pin_id: u64,
}

pub struct PinBurned {
    pin_owner: Identity,
    pin_id: u64,
}
