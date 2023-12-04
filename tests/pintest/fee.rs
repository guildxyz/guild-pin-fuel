use crate::{check_error, check_event};
use guild_pin_contract::contract::{FeeChanged, GuildPinContract};
use guild_pin_contract::parameters::ParametersBuilder;

#[tokio::test]
async fn set_fee_success() {
    let new_fee = 200;
    let parameters = ParametersBuilder::new().build().await;
    let contract = GuildPinContract::init(&parameters).await;

    let contract_fee = contract.fee().await.unwrap();
    assert_eq!(contract_fee, parameters.fee);

    let response = contract.set_fee(&parameters.owner, new_fee).await.unwrap();
    check_event(
        response,
        FeeChanged {
            old: parameters.fee,
            new: new_fee,
        },
    );

    let contract_fee = contract.fee().await.unwrap();
    assert_eq!(contract_fee, new_fee);
}

#[tokio::test]
async fn set_fee_fails() {
    let parameters = ParametersBuilder::new().build().await;
    let contract = GuildPinContract::init(&parameters).await;

    let error = contract.set_fee(&parameters.bob, 0).await.unwrap_err();
    check_error(error, "NotOwner");

    let contract_fee = contract.fee().await.unwrap();
    assert_eq!(contract_fee, parameters.fee);
}
