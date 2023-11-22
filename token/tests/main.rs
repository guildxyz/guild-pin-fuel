mod setup;

use fuels::types::{Address, Identity};
use setup::{PinMinted, TestContract};

#[tokio::test]
async fn contract_initialized() {
    let contract = TestContract::new().await;
    let owner = contract.owner().await;
    assert_eq!(owner, contract.owner.address().into());
    // cannot initialize again
    let call_result = contract.contract.methods().initialize().call().await;
    assert!(call_result.is_err());
}

#[tokio::test]
async fn can_mint() {
    let contract = TestContract::new().await;
    let response = contract
        .mint(&contract.owner, contract.user_0.address().into())
        .await;

    let events = response.decode_logs_with_type::<PinMinted>().unwrap();
    assert_eq!(
        events,
        vec![PinMinted {
            recipient: Identity::Address(Address::from(contract.user_0.address())),
            pin_id: 0
        }]
    );
}
