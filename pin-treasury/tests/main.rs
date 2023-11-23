mod setup;

use fuels::types::{errors::Error, Address, Identity};
use setup::{FeeSet, OwnerSet, TestContract, TreasurySet};

fn check_error(error: Error, expected: &str) {
    match error {
        Error::RevertTransactionError { reason, .. } => {
            assert_eq!(reason, expected);
        }
        _ => panic!("invalid error type"),
    }
}

#[tokio::test]
async fn contract_initialized() {
    let contract = TestContract::new().await;
    let owner = contract.owner().await;
    let treasury = contract.treasury().await;
    let fee = contract.fee().await;
    assert_eq!(owner, contract.owner.address().into());
    assert_eq!(treasury, contract.account_0.address().into());
    assert_eq!(fee, 100);
    // cannot initialize again
    let call_result = contract.contract.methods().initialize().call().await;
    check_error(call_result.unwrap_err(), "AlreadyInitialized");
}

#[tokio::test]
async fn set_owner() {
    let contract = TestContract::new().await;
    let account_0: Address = contract.account_0.address().into();
    let account_1: Address = contract.account_1.address().into();
    contract
        .set_owner(&contract.owner, account_0)
        .await
        .unwrap();
    assert_eq!(contract.owner().await, account_0);
    let response = contract.set_owner(&contract.owner, account_1).await;
    check_error(response.unwrap_err(), "NotOwner");
    let response = contract
        .set_owner(&contract.account_0, account_1)
        .await
        .unwrap();
    assert_eq!(contract.owner().await, account_1);
    let events = response.decode_logs_with_type::<OwnerSet>().unwrap();
    assert_eq!(
        events,
        vec![OwnerSet {
            old: Identity::Address(account_0),
            new: Identity::Address(account_1)
        }]
    );
}

#[tokio::test]
async fn set_treasury() {
    let contract = TestContract::new().await;
    let owner: Address = contract.owner.address().into();
    let account_0: Address = contract.account_0.address().into();
    let account_1: Address = contract.account_1.address().into();
    contract
        .set_treasury(&contract.owner, account_1)
        .await
        .unwrap();
    assert_eq!(contract.treasury().await, account_1);
    let response = contract.set_treasury(&contract.account_0, account_0).await;
    check_error(response.unwrap_err(), "NotOwner");
    let response = contract.set_treasury(&contract.owner, owner).await.unwrap();
    assert_eq!(contract.treasury().await, owner);
    let events = response.decode_logs_with_type::<TreasurySet>().unwrap();
    assert_eq!(
        events,
        vec![TreasurySet {
            old: Identity::Address(account_1),
            new: Identity::Address(owner)
        }]
    );
}

#[tokio::test]
async fn set_fee() {
    let contract = TestContract::new().await;
    contract.set_fee(&contract.owner, 200).await.unwrap();
    assert_eq!(contract.fee().await, 200);
    let response = contract.set_fee(&contract.account_0, 1000).await;
    check_error(response.unwrap_err(), "NotOwner");
    let response = contract.set_fee(&contract.owner, 150).await.unwrap();
    assert_eq!(contract.fee().await, 150);
    let events = response.decode_logs_with_type::<FeeSet>().unwrap();
    assert_eq!(events, vec![FeeSet { old: 200, new: 150 }]);
}
