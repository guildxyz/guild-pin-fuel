use crate::{check_error, check_event};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use fuels::types::Address;
use guild_pin_contract::contract::{GuildPinContract, PinMinted};
use guild_pin_contract::metadata::*;
use guild_pin_contract::parameters::ParametersBuilder;
use guild_pin_contract::utils::ClaimBuilder;

#[tokio::test]
async fn metadata_ok() {
    let parameters = ParametersBuilder::new().test().await;
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

    let mut header = contract.encoded_metadata(0).await.unwrap();
    let encoded_metadata = header.split_off(29);
    assert_eq!(header, "data:application/json;base64,");
    let decoded_metadata = String::from_utf8(STANDARD.decode(encoded_metadata).unwrap()).unwrap();
    let metadata = contract.metadata(0).await.unwrap();
    println!("{}", metadata);
    assert_eq!(metadata, decoded_metadata);
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

#[tokio::test]
async fn metadata_nonexistent_fails() {
    let parameters = ParametersBuilder::new().test().await;
    let contract = GuildPinContract::init(&parameters).await;

    // NOTE since contract.metadata() is a wrapper around simulating the call, it will return with
    // a "Revert" error, instead of the actual "PinIdDoesNotExist" error. Thus, the contract is
    // called in this raw form for test's sake.
    let error = contract
        .inner()
        .methods()
        .metadata(0)
        .call()
        .await
        .unwrap_err();
    check_error(error, "PinIdDoesNotExist");

    let alice: Address = parameters.alice.address().into();
    let clp = ClaimBuilder::new(alice, contract.contract_id()).build();

    let signature = parameters.sign_claim(&clp);
    contract
        .claim(&parameters.alice, clp.clone(), signature)
        .await
        .unwrap();

    contract.metadata(0).await.unwrap();

    contract.burn(&parameters.alice, 0).await.unwrap();

    let error = contract
        .inner()
        .methods()
        .metadata(0)
        .call()
        .await
        .unwrap_err();
    check_error(error, "PinIdDoesNotExist");
}
