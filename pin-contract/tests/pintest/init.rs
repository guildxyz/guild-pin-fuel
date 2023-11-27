use crate::contract::{ContractInitialized, GuildPinContract};
use crate::parameters::Parameters;
use crate::{check_error, check_event};

#[tokio::test]
async fn init_by_owner_success() {
    let fee = 10;
    let parameters = Parameters::new(fee).await;
    let contract = GuildPinContract::new(&parameters).await;

    let response = contract.initialize(&parameters.owner).await.unwrap();
    check_event(
        response,
        ContractInitialized {
            owner: parameters.owner_id(),
            signer: parameters.signer_evm(),
            treasury: parameters.treasury_id(),
            fee,
        },
    );
}

#[tokio::test]
async fn init_by_random_success() {
    let fee = 10;
    let parameters = Parameters::new(fee).await;
    let contract = GuildPinContract::new(&parameters).await;

    let response = contract.initialize(&parameters.alice).await.unwrap();
    let events = response
        .decode_logs_with_type::<ContractInitialized>()
        .unwrap();
    assert_eq!(
        events,
        vec![ContractInitialized {
            owner: parameters.owner_id(),
            signer: parameters.signer_evm(),
            treasury: parameters.treasury_id(),
            fee,
        }]
    );

    // cannot initialize twice
    let error = contract.initialize(&parameters.owner).await.unwrap_err();
    check_error(error, "AlreadyInitialized");
}
