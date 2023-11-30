use crate::contract::{GuildAction, GuildPinContract, PinMinted};
use crate::parameters::ParametersBuilder;
use crate::utils::ClaimBuilder;
use crate::{check_error, check_event};
use fuels::types::{Address, AssetId, ContractId};
use serde::{Deserialize, Serialize};

#[tokio::test]
async fn metadata_ok() {
    let parameters = ParametersBuilder::new().build().await;
    let contract = GuildPinContract::init(&parameters).await;

    let alice: Address = parameters.alice.address().into();
    let clp = ClaimBuilder::new(alice, contract.contract_id()).build();

    let signature = parameters.sign_claim(&clp);
    let response = contract
        .claim(&parameters.alice, clp.clone(), signature)
        .await
        .unwrap();

    check_event(
        response,
        PinMinted {
            recipient: alice,
            pin_id: 0,
        },
    );

    let metadata = contract.metadata(0).await.unwrap();
    println!("{}", metadata);
    let token_uri: TokenUri = serde_json::from_str(&metadata).unwrap();
    assert_eq!(token_uri.name, Action::Joined);
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Action {
    Joined,
    #[serde(rename = "Admin of")]
    Admin,
    #[serde(rename = "Owner of")]
    Owner,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenUri {
    pub name: Action,
    pub description: String,
    pub image: String,
    pub attributes: [Attribute; 6],
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(tag = "trait_type", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum Attribute {
    Type(Action),
    GuildId(u64),
    UserId(u64),
    Rank(u64),
    ActionDate(u64),
    MintDate(u64),
}
