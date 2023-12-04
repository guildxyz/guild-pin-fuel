use crate::contract::{ClaimParameters, GuildAction};
use fuels::types::{Address, ContractId, Identity, SizedAsciiString};
use sha3::digest::Digest;
use std::time::{SystemTime, UNIX_EPOCH};

pub const CID64: &str = "abcdefghijklmnopqrstuvxyzabcdefghijklmnopqrstuvxyzabcdefghijklmn";

pub fn keccak256<T: AsRef<[u8]>>(input: T) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut hasher = sha3::Keccak256::new();
    hasher.update(input);
    hasher.finalize_into((&mut output).into());
    output
}

pub fn hash_params(params: &ClaimParameters) -> [u8; 32] {
    keccak256(params_to_bytes(params))
}

// NOTE mimicking sway-lib-std/src/identity.sw hash impl
fn hash_identity(identity: &Identity, bytes: &mut Vec<u8>) {
    match identity {
        Identity::Address(address) => {
            bytes.push(0);
            bytes.extend_from_slice(address.as_slice());
        }
        Identity::ContractId(contract_id) => {
            bytes.push(1);
            bytes.extend_from_slice(contract_id.as_slice());
        }
    }
}

fn params_to_bytes(params: &ClaimParameters) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(params.recipient.as_slice());
    bytes.push(action_byte(&params.action));
    bytes.extend_from_slice(&params.user_id.to_be_bytes());
    bytes.extend_from_slice(&params.guild_id.to_be_bytes());
    bytes.extend_from_slice(params.guild_name.as_ref());
    bytes.extend_from_slice(&params.created_at.to_be_bytes());
    bytes.extend_from_slice(&params.signed_at.to_be_bytes());
    bytes.extend_from_slice(&params.chain_id.to_be_bytes());
    bytes.extend_from_slice(params.cid.as_ref());
    hash_identity(&params.admin_treasury, &mut bytes);
    bytes.extend_from_slice(&params.admin_fee.to_be_bytes());
    bytes.extend_from_slice(params.contract_id.as_ref());
    bytes
}

fn action_byte(action: &GuildAction) -> u8 {
    match action {
        GuildAction::Joined => 0,
        GuildAction::Owner => 1,
        GuildAction::Admin => 2,
    }
}

pub fn to_tai64_timestamp(unix_seconds: u64) -> u64 {
    unix_seconds + (1u64 << 62) + 10u64
}

pub fn from_tai64_timestamp(tai_seconds: u64) -> u64 {
    tai_seconds - (1u64 << 62) - 10u64
}

pub struct ClaimBuilder {
    pub recipient: Address,
    pub action: GuildAction,
    pub user_id: u64,
    pub guild_id: u64,
    pub guild_name: SizedAsciiString<64>,
    pub created_at: u64,
    pub signed_at: u64,
    pub chain_id: u64,
    pub cid: SizedAsciiString<64>,
    pub admin_treasury: Identity,
    pub admin_fee: u64,
    pub contract_id: ContractId,
}

impl ClaimBuilder {
    pub fn new(recipient: Address, contract_id: ContractId) -> Self {
        Self {
            recipient,
            action: GuildAction::Joined,
            user_id: 100,
            guild_id: 1234,
            guild_name: SizedAsciiString::new_with_right_whitespace_padding(
                "MyAwesomeGuild".to_string(),
            )
            .unwrap(),
            created_at: 100_000,
            signed_at: to_tai64_timestamp(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ),
            cid: SizedAsciiString::new_with_right_whitespace_padding(CID64.to_string()).unwrap(),
            chain_id: 9999,
            admin_treasury: Identity::ContractId(contract_id),
            admin_fee: 0,
            contract_id,
        }
    }

    pub fn action(mut self, action: GuildAction) -> Self {
        self.action = action;
        self
    }

    pub fn admin_treasury(mut self, treasury: Identity) -> Self {
        self.admin_treasury = treasury;
        self
    }

    pub fn admin_fee(mut self, fee: u64) -> Self {
        self.admin_fee = fee;
        self
    }

    pub fn user_id(mut self, user_id: u64) -> Self {
        self.user_id = user_id;
        self
    }

    pub fn chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = chain_id;
        self
    }

    pub fn guild_id(mut self, guild_id: u64) -> Self {
        self.guild_id = guild_id;
        self
    }

    pub fn signed_at(mut self, signed_at: u64) -> Self {
        self.signed_at = signed_at;
        self
    }

    pub fn build(self) -> ClaimParameters {
        ClaimParameters {
            recipient: self.recipient,
            action: self.action,
            user_id: self.user_id,
            guild_id: self.guild_id,
            guild_name: self.guild_name,
            created_at: self.created_at,
            signed_at: self.signed_at,
            chain_id: self.chain_id,
            cid: self.cid,
            admin_treasury: self.admin_treasury,
            admin_fee: self.admin_fee,
            contract_id: self.contract_id,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn endianness_test_for_hashing() {
        // NOTE hashes were taken from sway-lib-std/src/hash.sw
        let output = keccak256(0u64.to_be_bytes());
        assert_eq!(
            hex::encode(output),
            "011b4d03dd8c01f1049143cf9c4c817e4b167f1d1b83e5c6f0f10d89ba1e7bce"
        );
        let output = keccak256(1u64.to_be_bytes());
        assert_eq!(
            hex::encode(output),
            "6c31fc15422ebad28aaf9089c306702f67540b53c7eea8b7d2941044b027100f"
        );
    }

    #[test]
    fn tai64() {
        let unix = 1234567890;
        let tai = to_tai64_timestamp(unix);
        assert_eq!(from_tai64_timestamp(tai), unix);
    }
}
