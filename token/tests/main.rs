mod setup;

use fuels::types::{errors::Error, Address, Identity};
use setup::{PinMinted, TestContract};

#[tokio::test]
async fn contract_initialized() {
    let contract = TestContract::new().await;
    let owner = contract.owner().await;
    assert_eq!(owner, contract.owner.address().into());
    // cannot initialize again
    let call_result = contract.contract.methods().initialize().call().await;
    match call_result.unwrap_err() {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, "AlreadyInitialized");
        }
        _ => panic!("invalid error type"),
    }
}

#[tokio::test]
async fn can_mint() {
    let contract = TestContract::new().await;
    let recipient: Address = contract.user_0.address().into();
    let recipient_id = Identity::Address(recipient);

    // check initial storage
    let balance = contract.balance(recipient).await.value;
    assert_eq!(balance, 0);
    let pin_owner = contract.pin_owner(0).await.value;
    assert!(pin_owner.is_none());

    // mint token
    let response = contract.mint(&contract.owner, recipient).await;

    // check modified storage
    let balance = contract.balance(recipient).await.value;
    assert_eq!(balance, 1);
    let pin_owner = contract.pin_owner(0).await.value;
    assert_eq!(pin_owner.as_ref(), Some(&recipient_id));

    // check emitted events
    let events = response.decode_logs_with_type::<PinMinted>().unwrap();
    assert_eq!(
        events,
        vec![PinMinted {
            recipient: recipient_id,
            pin_id: 0
        }]
    );
}
