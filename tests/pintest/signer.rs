use crate::{check_error, check_event};
use guild_pin_contract::contract::{GuildPinContract, SignerChanged};
use guild_pin_contract::parameters::ParametersBuilder;

#[tokio::test]
async fn set_signer_success() {
    let parameters = ParametersBuilder::new().build().await;
    let contract = GuildPinContract::init(&parameters).await;

    let owner = contract.owner().await.unwrap();
    assert_eq!(owner, parameters.owner_id());

    let signer = contract.signer().await.unwrap();
    assert_eq!(signer, parameters.signer_evm());
    assert_eq!(signer.value().0[12..], parameters.signer.address());

    let response = contract
        .set_signer(&parameters.owner, parameters.signer_alt_evm())
        .await
        .unwrap();
    check_event(
        response,
        SignerChanged {
            old: parameters.signer_evm(),
            new: parameters.signer_alt_evm(),
        },
    );

    let signer = contract.signer().await.unwrap();
    assert_eq!(signer, parameters.signer_alt_evm());
}

#[tokio::test]
async fn set_signer_fails() {
    let parameters = ParametersBuilder::new().build().await;
    let contract = GuildPinContract::init(&parameters).await;

    let error = contract
        .set_signer(&parameters.bob, parameters.signer_alt_evm())
        .await
        .unwrap_err();
    check_error(error, "NotOwner");

    let signer = contract.signer().await.unwrap();
    assert_eq!(signer, parameters.signer_evm());
}
