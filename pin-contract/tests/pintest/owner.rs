use crate::check_error;
use crate::contract::GuildPinContract;
use crate::parameters::Parameters;

use fuels::types::errors::Error;

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
}
