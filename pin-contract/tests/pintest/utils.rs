use crate::contract::{ClaimParameters, GuildAction};
use fuels::types::{ContractId, Identity};
use sha3::digest::Digest;

pub fn keccak256<T: AsRef<[u8]>>(input: T) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut hasher = sha3::Keccak256::new();
    hasher.update(input);
    hasher.finalize_into((&mut output).into());
    output
}

pub fn hash_params(params: &ClaimParameters) -> [u8; 32] {
    keccak256(&params_to_bytes(params))
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
    bytes.extend_from_slice(params.cid.as_ref());
    bytes.extend_from_slice(params.admin_treasury.as_ref());
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
}
