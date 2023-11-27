use fuels::prelude::{launch_custom_provider_and_get_wallets, WalletUnlocked, WalletsConfig};
use fuels::types::Bits256;
use signrs::eth::EthSigner;

pub struct Parameters {
    pub contract: WalletUnlocked,
    pub owner: WalletUnlocked,
    pub treasury: WalletUnlocked,
    pub signer: EthSigner,
    pub fee: u64,
    pub alice: WalletUnlocked,
    pub bob: WalletUnlocked,
    pub charlie: WalletUnlocked,
}

impl Parameters {
    pub async fn new(fee: u64) -> Self {
        let number_of_wallets = 6;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000_000;
        let wallet_config = WalletsConfig::new(
            Some(number_of_wallets),
            Some(coins_per_wallet),
            Some(amount_per_coin),
        );
        let mut wallets = launch_custom_provider_and_get_wallets(wallet_config, None, None)
            .await
            .unwrap();

        Self {
            contract: wallets.pop().unwrap(),
            owner: wallets.pop().unwrap(),
            treasury: wallets.pop().unwrap(),
            signer: EthSigner::new(&[3u8; 32]),
            fee,
            alice: wallets.pop().unwrap(),
            bob: wallets.pop().unwrap(),
            charlie: wallets.pop().unwrap(),
        }
    }

    pub fn signer_b256(&self) -> Bits256 {
        let mut b256 = Bits256::zeroed();
        b256.0[..20].copy_from_slice(&self.signer.address());
        b256
    }
}
