library;

pub enum TokenError {
    AlreadyInitialized: (),
    AlreadyMinted: (),
    AlreadyBurned: (),
    InvalidAmount: (),
    InvalidSubId: (),
    InvalidAssetId: (),
    PinIdDoesNotExist: (),
}
