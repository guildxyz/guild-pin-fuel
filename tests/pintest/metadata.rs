use crate::check_event;
use crate::contract::{GuildPinContract, PinMinted};
use crate::parameters::ParametersBuilder;
use crate::utils::ClaimBuilder;
use fuels::types::Address;
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
    assert_eq!(
        token_uri.description,
        format!(
            "This is an onchain proof that you joined {} on Guild.xyz",
            clp.guild_name.to_trimmed_str()
        )
    );
    assert_eq!(
        token_uri.image,
        format!("ipfs://{}", clp.cid.to_trimmed_str())
    );
    assert!(token_uri
        .attributes
        .iter()
        .any(|attr| attr == &Attribute::Type(Action::Joined)));
    assert!(token_uri
        .attributes
        .iter()
        .any(|attr| attr == &Attribute::GuildId(clp.guild_id)));
    assert!(token_uri
        .attributes
        .iter()
        .any(|attr| attr == &Attribute::UserId(clp.user_id)));
    assert!(token_uri
        .attributes
        .iter()
        .any(|attr| attr == &Attribute::Rank(0)));
    assert!(token_uri
        .attributes
        .iter()
        .any(|attr| attr == &Attribute::ActionDate(clp.created_at.to_string())));
    // cannot test for the exact mint date but we can check whether it
    // occurred in the previous 30 seconds
    let mint_date = token_uri
        .attributes
        .iter()
        .filter_map(|attr| attr.mint_date())
        .collect::<Vec<u64>>()[0];
    let now = parameters.timestamp().await;
    assert!(mint_date <= now && mint_date >= now - 30);

    let json_value: serde_json::Value = serde_json::from_str(&metadata).unwrap();
    assert_eq!(json_value["attributes"][4]["display_type"], "date");
    assert_eq!(json_value["attributes"][5]["display_type"], "date");
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "trait_type", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum Attribute {
    Type(Action),
    GuildId(u64),
    UserId(u64),
    Rank(u64),
    ActionDate(String), // dates are formatted as string
    MintDate(String),   // dates are formatted as string
}

impl Attribute {
    fn mint_date(&self) -> Option<u64> {
        if let Self::MintDate(d) = self {
            d.parse::<u64>().ok()
        } else {
            None
        }
    }
}