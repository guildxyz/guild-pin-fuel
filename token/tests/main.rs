mod setup;

use setup::TestContract;

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
    contract
        .mint(&contract.owner, contract.user_0.address().into())
        .await;
}
