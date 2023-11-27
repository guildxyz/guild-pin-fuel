use crate::contract::{GuildPinContract, TreasuryChanged};
use crate::parameters::Parameters;
use crate::{check_error, check_event};

#[tokio::test]
async fn set_treasury_success() {
    let parameters = Parameters::new(10).await;
    let contract = GuildPinContract::init(&parameters).await;

    let treasury = contract.treasury().await.unwrap();
    assert_eq!(treasury, parameters.treasury_id());

    let response = contract
        .set_treasury(&parameters.owner, parameters.bob_id())
        .await
        .unwrap();
    check_event(
        response,
        TreasuryChanged {
            old: parameters.treasury_id(),
            new: parameters.bob_id(),
        },
    );

    let treasury = contract.treasury().await.unwrap();
    assert_eq!(treasury, parameters.bob_id());
}

#[tokio::test]
async fn set_treasury_fails() {
    let parameters = Parameters::new(10).await;
    let contract = GuildPinContract::init(&parameters).await;

    let error = contract
        .set_treasury(&parameters.bob, parameters.bob_id())
        .await
        .unwrap_err();
    check_error(error, "NotOwner");

    let treasury = contract.treasury().await.unwrap();
    assert_eq!(treasury, parameters.treasury_id());
}
