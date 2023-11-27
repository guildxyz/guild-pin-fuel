use crate::contract::{GuildPinContract, OwnerChanged};
use crate::parameters::Parameters;
use crate::{check_error, check_event};

#[tokio::test]
async fn set_owner_success() {
    let fee = 10;
    let parameters = Parameters::new(fee).await;
    let contract = GuildPinContract::new(&parameters).await;

    let error = contract.owner().await.unwrap_err();
    assert_eq!(error.to_string(), "Invalid data: NotInitialized");

    contract.initialize(&parameters.bob).await.unwrap();
    let owner = contract.owner().await.unwrap();
    assert_eq!(owner, parameters.owner_id());

    let response = contract
        .set_owner(&parameters.owner, parameters.alice_id())
        .await
        .unwrap();
    check_event(
        response,
        OwnerChanged {
            old: parameters.owner_id(),
            new: parameters.alice_id(),
        },
    );
    let owner = contract.owner().await.unwrap();
    assert_eq!(owner, parameters.alice_id());
}

#[tokio::test]
async fn set_owner_fails() {
    let parameters = Parameters::new(10).await;
    let contract = GuildPinContract::new(&parameters).await;

    // try to set owner before initialization
    let error = contract
        .set_owner(&parameters.owner, parameters.owner_id())
        .await
        .unwrap_err();
    check_error(error, "NotOwner");

    contract.initialize(&parameters.owner).await.unwrap();

    let owner = contract.owner().await.unwrap();
    assert_eq!(owner, parameters.owner_id());

    let error = contract
        .set_owner(&parameters.charlie, parameters.charlie_id())
        .await
        .unwrap_err();
    check_error(error, "NotOwner");

    let owner = contract.owner().await.unwrap();
    assert_eq!(owner, parameters.owner_id());
}
