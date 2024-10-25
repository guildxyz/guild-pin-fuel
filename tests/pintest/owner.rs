use crate::{check_error, check_event};
use guild_pin_contract::contract::{GuildPinContract, OwnershipTransferred};
use guild_pin_contract::parameters::ParametersBuilder;

#[tokio::test]
async fn set_owner_success() {
    let parameters = ParametersBuilder::new().test().await;
    let contract = GuildPinContract::deploy(&parameters).await;

    let error = contract.owner().await.unwrap_err();
    assert_eq!(error.to_string(), "io: NotInitialized");

    contract.initialize(&parameters.bob).await.unwrap();
    let owner = contract.owner().await.unwrap();
    assert_eq!(owner, parameters.owner_id());

    let response = contract
        .set_owner(&parameters.owner, parameters.alice_id())
        .await
        .unwrap();
    check_event(
        response,
        OwnershipTransferred {
            previous_owner: parameters.owner_id(),
            new_owner: parameters.alice_id(),
        },
    );
    let owner = contract.owner().await.unwrap();
    assert_eq!(owner, parameters.alice_id());
}

#[tokio::test]
async fn set_owner_fails() {
    let parameters = ParametersBuilder::new().test().await;
    let contract = GuildPinContract::deploy(&parameters).await;

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
