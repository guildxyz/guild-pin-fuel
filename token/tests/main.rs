mod setup;

use fuels::prelude::AssetId;
use fuels::types::{errors::Error, Address, Bits256, Identity};
use setup::{OwnerSet, PinBurned, PinMinted, TestContract};

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
    assert_eq!(owner, contract.owner.address().into());
    // cannot initialize again
    let call_result = contract.contract.methods().initialize().call().await;
    check_error(call_result.unwrap_err(), "AlreadyInitialized");
}

#[tokio::test]
async fn mint_success() {
    let contract = TestContract::new().await;
    let recipient: Address = contract.user_0.address().into();
    let recipient_id = Identity::Address(recipient);

    // check initial storage
    let balance = contract.balance(recipient).await.value;
    assert_eq!(balance, 0);
    let pin_owner = contract.pin_owner(0).await.value;
    assert!(pin_owner.is_none());

    // mint token
    let response = contract.mint(&contract.owner, recipient).await.unwrap();

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

#[tokio::test]
async fn mint_failure() {
    let contract = TestContract::new().await;
    let response = contract
        .mint(&contract.user_1, contract.user_1.address().into())
        .await;
    check_error(response.unwrap_err(), "NotOwner");

    let response = contract
        .unsafe_mint(
            &contract.owner,
            contract.user_1.address().into(),
            Bits256::zeroed(),
            2,
        )
        .await;
    check_error(response.unwrap_err(), "InvalidAmount");

    let response = contract
        .unsafe_mint(
            &contract.owner,
            contract.user_1.address().into(),
            Bits256([1u8; 32]),
            1,
        )
        .await;
    check_error(response.unwrap_err(), "InvalidSubId");
}

#[tokio::test]
async fn set_owner() {
    let contract = TestContract::new().await;
    let user_0: Address = contract.user_0.address().into();
    let user_1: Address = contract.user_0.address().into();
    contract.set_owner(&contract.owner, user_0).await.unwrap();
    assert_eq!(contract.owner().await, user_0);
    let response = contract.set_owner(&contract.owner, user_1).await;
    check_error(response.unwrap_err(), "NotOwner");
    let response = contract.set_owner(&contract.user_0, user_1).await.unwrap();
    assert_eq!(contract.owner().await, user_1);
    let events = response.decode_logs_with_type::<OwnerSet>().unwrap();
    assert_eq!(
        events,
        vec![OwnerSet {
            old: Identity::Address(user_0),
            new: Identity::Address(user_1)
        }]
    );
}

#[tokio::test]
async fn burn_success() {
    let contract = TestContract::new().await;
    let recipient_0: Address = contract.user_0.address().into();
    let recipient_1: Address = contract.user_1.address().into();
    let recipient_id_0 = Identity::Address(recipient_0);
    let recipient_id_1 = Identity::Address(recipient_1);

    contract.mint(&contract.owner, recipient_0).await.unwrap();
    contract.mint(&contract.owner, recipient_0).await.unwrap();
    contract.mint(&contract.owner, recipient_1).await.unwrap();
    contract.mint(&contract.owner, recipient_0).await.unwrap();
    contract.mint(&contract.owner, recipient_1).await.unwrap();

    let balance = contract.balance(recipient_0).await.value;
    assert_eq!(balance, 3);
    let balance = contract.balance(recipient_1).await.value;
    assert_eq!(balance, 2);
    let pin_owner = contract.pin_owner(0).await.value;
    assert_eq!(pin_owner.as_ref(), Some(&recipient_id_0));
    let pin_owner = contract.pin_owner(4).await.value;
    assert_eq!(pin_owner.as_ref(), Some(&recipient_id_1));
    let pin_owner = contract.pin_owner(5).await.value;
    assert!(pin_owner.is_none());

    assert_eq!(contract.total_supply().await, 5);
    assert_eq!(contract.total_minted().await, 5);

    // user_0 burns their first pin
    let response = contract.burn(&contract.user_0, 0).await.unwrap();
    let events = response.decode_logs_with_type::<PinBurned>().unwrap();
    assert_eq!(
        events,
        vec![PinBurned {
            pin_owner: recipient_id_0,
            pin_id: 0
        }]
    );

    // user_1 burns their last pin
    let response = contract.burn(&contract.user_1, 4).await.unwrap();
    let events = response.decode_logs_with_type::<PinBurned>().unwrap();
    assert_eq!(
        events,
        vec![PinBurned {
            pin_owner: recipient_id_1,
            pin_id: 4
        }]
    );

    assert_eq!(contract.total_supply().await, 3);
    assert_eq!(contract.total_minted().await, 5);

    let balance = contract.balance(recipient_0).await.value;
    assert_eq!(balance, 2);
    let balance = contract.balance(recipient_1).await.value;
    assert_eq!(balance, 1);
    let pin_owner = contract.pin_owner(0).await.value;
    assert!(pin_owner.is_none());
    let pin_owner = contract.pin_owner(4).await.value;
    assert!(pin_owner.is_none());

    contract.mint(&contract.owner, recipient_1).await.unwrap();

    assert_eq!(contract.total_supply().await, 4);
    assert_eq!(contract.total_minted().await, 6);

    let balance = contract.balance(recipient_0).await.value;
    assert_eq!(balance, 2);
    let balance = contract.balance(recipient_1).await.value;
    assert_eq!(balance, 2);
}

#[tokio::test]
async fn burn_failure() {
    let contract = TestContract::new().await;
    let recipient: Address = contract.user_0.address().into();

    contract.mint(&contract.owner, recipient).await.unwrap();

    let response = contract.burn(&contract.owner, 0).await;
    check_error(response.unwrap_err(), "NotOwner");

    let response = contract.burn(&contract.user_0, 1).await;
    check_error(response.unwrap_err(), "PinIdDoesNotExist");

    contract.burn(&contract.user_0, 0).await.unwrap();

    let response = contract.burn(&contract.user_0, 0).await;
    check_error(response.unwrap_err(), "AlreadyBurned");
}

#[tokio::test]
async fn src20() {
    let contract = TestContract::new().await;
    let asset_id = contract.contract.contract_id().asset_id(&Bits256::zeroed());

    let total_assets = contract
        .contract
        .methods()
        .total_assets()
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(total_assets, 1);

    let name = contract
        .contract
        .methods()
        .name(AssetId::zeroed())
        .call()
        .await
        .unwrap()
        .value;
    assert!(name.is_none());

    let symbol = contract
        .contract
        .methods()
        .symbol(AssetId::zeroed())
        .call()
        .await
        .unwrap()
        .value;
    assert!(symbol.is_none());

    let decimals = contract
        .contract
        .methods()
        .decimals(AssetId::zeroed())
        .call()
        .await
        .unwrap()
        .value;
    assert!(decimals.is_none());

    let total_supply = contract
        .contract
        .methods()
        .total_supply(AssetId::zeroed())
        .call()
        .await
        .unwrap()
        .value;
    assert!(total_supply.is_none());

    let name = contract
        .contract
        .methods()
        .name(asset_id)
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(name, Some("Guild Pin".to_string()));

    let symbol = contract
        .contract
        .methods()
        .symbol(asset_id)
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(symbol, Some("GUILD".to_string()));

    let decimals = contract
        .contract
        .methods()
        .decimals(asset_id)
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(decimals, Some(0));

    let total_supply = contract
        .contract
        .methods()
        .total_supply(asset_id)
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(total_supply, Some(0));
}
