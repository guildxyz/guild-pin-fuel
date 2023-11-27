use crate::contract::{FeeChanged, GuildPinContract};
use crate::parameters::Parameters;
use crate::{check_error, check_event};

#[tokio::test]
async fn set_fee_success() {
    let fee = 10;
    let new_fee = 20;
    let parameters = Parameters::new(fee).await;
    let contract = GuildPinContract::init(&parameters).await;

    let contract_fee = contract.fee().await.unwrap();
    assert_eq!(contract_fee, fee);

    let response = contract.set_fee(&parameters.owner, new_fee).await.unwrap();
    check_event(
        response,
        FeeChanged {
            old: fee,
            new: new_fee,
        },
    );

    let contract_fee = contract.fee().await.unwrap();
    assert_eq!(contract_fee, new_fee);
}

#[tokio::test]
async fn set_fee_fails() {
    let fee = 10;
    let parameters = Parameters::new(fee).await;
    let contract = GuildPinContract::init(&parameters).await;

    let error = contract.set_fee(&parameters.bob, 0).await.unwrap_err();
    check_error(error, "NotOwner");

    let contract_fee = contract.fee().await.unwrap();
    assert_eq!(contract_fee, fee);
}
