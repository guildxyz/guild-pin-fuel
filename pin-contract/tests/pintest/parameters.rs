use fuels::prelude::WalletUnlocked;

type EvmSigner = ();

pub struct Parameters {
    pub deployer: WalletUnlocked,
    pub owner: WalletUnlocked,
    pub treasury: WalletUnlocked,
    pub signer: EvmSigner,
    pub fee: u64,
}

pub struct Users {
    pub alice: WalletUnlocked,
    pub bob: WalletUnlocked,
    pub charlie: WalletUnlocked,
}
