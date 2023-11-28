use crate::contract::{ClaimParameters, GuildAction, GuildPinContract};
use crate::parameters::Parameters;
use crate::{check_error, check_event};
use fuels::types::{Identity, SizedAsciiString};

const CID64: &str = "abcdefghijklmnopqrstuvxyzabcdefghijklmnopqrstuvxyzabcdefghijklmn";

#[tokio::test]
async fn claim_successful() {
    let fee = 10;
    let parameters = Parameters::new(10).await;
    let contract = GuildPinContract::init(&parameters).await;

    let claim_params = ClaimParameters {
        recipient: parameters.alice.address().into(),
        action: GuildAction::Joined,
        user_id: 10,
        guild_id: 1234,
        guild_name: SizedAsciiString::new_with_right_whitespace_padding(
            "MyAwesomeGuild".to_string(),
        )
        .unwrap(),
        created_at: 1000,
        signed_at: 1000000000000,
        cid: SizedAsciiString::new_with_right_whitespace_padding(CID64.to_string()).unwrap(),
        admin_treasury: Identity::ContractId(contract.contract_id()),
        admin_fee: 0,
        contract_id: contract.contract_id(),
    };

    let signature = parameters.sign_claim(&claim_params);
    let response = contract
        .claim(&parameters.alice, claim_params, signature)
        .await
        .unwrap();
}
