use crate::contract::ClaimParameters;
use crate::utils::hash_params;
use fuels::prelude::{launch_custom_provider_and_get_wallets, WalletUnlocked, WalletsConfig};
use fuels::types::{Bits256, EvmAddress, Identity, B512};
use signrs::eth::EthSigner;
use signrs::eth::hash_eth_message;

pub struct Parameters {
    pub contract: WalletUnlocked,
    pub owner: WalletUnlocked,
    pub treasury: WalletUnlocked,
    pub signer: EthSigner,
    pub signer_alt: EthSigner,
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
            signer_alt: EthSigner::new(&[19u8; 32]),
            fee,
            alice: wallets.pop().unwrap(),
            bob: wallets.pop().unwrap(),
            charlie: wallets.pop().unwrap(),
        }
    }

    pub fn signer_b256(&self) -> Bits256 {
        let mut b256 = Bits256::zeroed();
        b256.0[12..].copy_from_slice(&self.signer.address());
        b256
    }

    pub fn signer_alt_b256(&self) -> Bits256 {
        let mut b256 = Bits256::zeroed();
        b256.0[12..].copy_from_slice(&self.signer_alt.address());
        b256
    }

    pub fn signer_evm(&self) -> EvmAddress {
        EvmAddress::from(self.signer_b256())
    }

    pub fn signer_alt_evm(&self) -> EvmAddress {
        EvmAddress::from(self.signer_alt_b256())
    }

    pub fn owner_id(&self) -> Identity {
        Identity::Address(self.owner.address().into())
    }

    pub fn treasury_id(&self) -> Identity {
        Identity::Address(self.treasury.address().into())
    }

    pub fn alice_id(&self) -> Identity {
        Identity::Address(self.alice.address().into())
    }

    pub fn bob_id(&self) -> Identity {
        Identity::Address(self.bob.address().into())
    }

    pub fn charlie_id(&self) -> Identity {
        Identity::Address(self.charlie.address().into())
    }

    pub fn sign_claim(&self, params: &ClaimParameters) -> B512 {
        _sign_claim(params, &self.signer)
    }

    pub fn sign_alt_claim(&self, params: &ClaimParameters) -> B512 {
        _sign_claim(params, &self.signer_alt)
    }
}

fn _sign_claim(params: &ClaimParameters, signer: &EthSigner) -> B512 {
    let hashed_params = hash_params(params);
    dbg!("{}", Bits256(hash_eth_message(hashed_params).into()));
    dbg!("{}", hex::encode(signer.address()));
    let signature = signer.sign(&hashed_params);
    let mut hi = Bits256::zeroed();
    let mut lo = Bits256::zeroed();
    hi.0.copy_from_slice(&signature[0..32]);
    lo.0.copy_from_slice(&signature[32..64]);
    B512::from((hi, lo))
}
