use crate::{check_error, check_event};
use guild_pin_contract::contract::{ContractInitialized, GuildPinContract};
use guild_pin_contract::parameters::ParametersBuilder;

#[tokio::test]
async fn init_by_owner_success() {
    let parameters = ParametersBuilder::new().build().await;
    let contract = GuildPinContract::deploy(&parameters).await;

    let response = contract.initialize(&parameters.owner).await.unwrap();
    check_event(
        response,
        ContractInitialized {
            owner: parameters.owner_id(),
            signer: parameters.signer_evm(),
            treasury: parameters.treasury_id(),
            fee: parameters.fee,
        },
    );

    // sanity src20 tests
    assert_eq!(contract.total_assets().await.unwrap(), 1);
    assert_eq!(contract.decimals().await.unwrap(), 0);
    assert_eq!(contract.name().await.unwrap(), "Guild Pin");
    assert_eq!(contract.symbol().await.unwrap(), "GUILD");
    assert_eq!(contract.total_supply().await.unwrap(), 0);
}

#[tokio::test]
async fn init_by_random_success() {
    let parameters = ParametersBuilder::new().build().await;
    let contract = GuildPinContract::deploy(&parameters).await;

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
            fee: parameters.fee,
        }]
    );

    // cannot initialize twice
    let error = contract.initialize(&parameters.owner).await.unwrap_err();
    check_error(error, "AlreadyInitialized");
}
