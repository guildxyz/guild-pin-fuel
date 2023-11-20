mod setup;

use setup::TestContract;

mod success {
    use super::*;
    #[tokio::test]
    async fn can_mint() {
        let contract = TestContract::new().await;
        contract
            .mint(
                contract.owner.address().into(),
                contract.user_0.address().into(),
            )
            .await;
    }
}
