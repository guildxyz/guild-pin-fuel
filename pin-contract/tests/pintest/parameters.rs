use crate::contract::ClaimParameters;
use crate::utils::hash_params;
use fuels::prelude::{
    launch_custom_provider_and_get_wallets, Provider, WalletUnlocked, WalletsConfig,
};
use fuels::types::{Bits256, EvmAddress, Identity, B512};
use signrs::eth::hash_eth_message;
use signrs::eth::EthSigner;

pub struct ParametersBuilder {
    pub fee: u64,
    pub genesis_balance: u64,
    pub signer_seed: [u8; 32],
    pub signer_alt_seed: [u8; 32],
}

impl Default for ParametersBuilder {
    fn default() -> Self {
        Self {
            fee: 10,
            genesis_balance: 1_000_000_000,
            signer_seed: [11u8; 32],
            signer_alt_seed: [22u8; 32],
        }
    }
}

impl ParametersBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_fee(mut self, fee: u64) -> Self {
        self.fee = fee;
        self
    }

    pub fn with_genesis_balance(mut self, genesis_balance: u64) -> Self {
        self.genesis_balance = genesis_balance;
        self
    }

    pub async fn build(self) -> Parameters {
        let number_of_wallets = 6;
        let coins_per_wallet = 1;
        let wallet_config = WalletsConfig::new(
            Some(number_of_wallets),
            Some(coins_per_wallet),
            Some(self.genesis_balance),
        );
        let mut wallets = launch_custom_provider_and_get_wallets(wallet_config, None, None)
            .await
            .unwrap();

        let contract = wallets.pop().unwrap();
        let provider = contract.provider().unwrap();

        Parameters {
            provider: provider.clone(),
            contract,
            owner: wallets.pop().unwrap(),
            treasury: wallets.pop().unwrap(),
            signer: EthSigner::new(&self.signer_seed),
            signer_alt: EthSigner::new(&self.signer_alt_seed),
            fee: self.fee,
            alice: wallets.pop().unwrap(),
            bob: wallets.pop().unwrap(),
            charlie: wallets.pop().unwrap(),
        }
    }
}

pub struct Parameters {
    pub provider: Provider,
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

// NOTE fuel uses the compact signature representation: https://eips.ethereum.org/EIPS/eip-2098
// I'm deliberately not using the sdk's fuel_crypto types (SecretKey, Signature, etc) because
// I want to mimic the backend signer, who uses an Ethers wallet to sign messages
fn _sign_claim(params: &ClaimParameters, signer: &EthSigner) -> B512 {
    let hashed_params = hash_params(params);
    let signature = signer.sign(&hashed_params);
    let parity = signature[64] - 27;
    debug_assert!(parity < 2);
    let mut hi = Bits256::zeroed();
    let mut lo = Bits256::zeroed();
    hi.0.copy_from_slice(&signature[0..32]);
    lo.0.copy_from_slice(&signature[32..64]);
    lo.0[0] += parity << 7;
    B512::from((hi, lo))
}
