use crate::{check_error, check_event};
use fuels::types::{Address, AssetId, ContractId};
use guild_pin_contract::contract::{GuildAction, GuildPinContract, PinMinted};
use guild_pin_contract::parameters::ParametersBuilder;
use guild_pin_contract::utils::ClaimBuilder;

#[tokio::test]
async fn claim_successful() {
    let fee = 20;
    let genesis_balance = 100;
    let parameters = ParametersBuilder::new()
        .fee(fee)
        .genesis_balance(genesis_balance)
        .test()
        .await;
    let contract = GuildPinContract::init(&parameters).await;

    let alice: Address = parameters.alice.address().into();
    let mut clp = ClaimBuilder::new(alice, contract.contract_id()).build();

    let signature = parameters.sign_claim(&clp);
    let response = contract
        .claim(&parameters.alice, clp.clone(), signature)
        .await
        .unwrap();

    check_event(
        response,
        PinMinted {
            recipient: alice,
            pin_id: 0,
        },
    );

    let balance = contract.balance_of(alice).await.unwrap();
    assert_eq!(balance, 1);

    let pin_owner = contract.pin_owner(0).await.unwrap();
    assert_eq!(pin_owner, Some(alice));

    let total_minted = contract.total_minted().await.unwrap();
    assert_eq!(total_minted, 1);

    let total_minted_per_guild = contract.total_minted_per_guild(clp.guild_id).await.unwrap();
    assert_eq!(total_minted_per_guild, 1);

    let pin_id = contract
        .pin_id_by_address(alice, clp.guild_id, clp.action.clone())
        .await
        .unwrap();
    assert_eq!(pin_id, Some(0));

    let pin_id = contract
        .pin_id_by_user_id(clp.user_id, clp.guild_id, clp.action.clone())
        .await
        .unwrap();
    assert_eq!(pin_id, Some(0));

    let pin_id = contract
        .pin_id_by_address(alice, 11, clp.action)
        .await
        .unwrap();
    assert_eq!(pin_id, None);

    let pin_id = contract
        .pin_id_by_user_id(clp.user_id, clp.guild_id, GuildAction::Owner)
        .await
        .unwrap();
    assert_eq!(pin_id, None);

    let base_balance = parameters
        .provider()
        .get_asset_balance(parameters.alice.address(), AssetId::BASE)
        .await
        .unwrap();
    assert_eq!(base_balance, genesis_balance - fee);

    let base_balance = parameters
        .provider()
        .get_asset_balance(parameters.treasury.address(), AssetId::BASE)
        .await
        .unwrap();
    assert_eq!(base_balance, genesis_balance + fee);

    let pin_balance = parameters
        .provider()
        .get_asset_balance(parameters.alice.address(), contract.asset_id())
        .await
        .unwrap();
    assert_eq!(pin_balance, 1);

    let pin_balances = parameters
        .provider()
        .get_contract_balances(contract.bech_contract_id())
        .await
        .unwrap();
    assert_eq!(pin_balances.get(&contract.asset_id()), Some(&0));

    // mint another pin with a different action
    // and admin treasury
    clp.action = GuildAction::Owner;
    clp.admin_treasury = parameters.charlie_id();
    clp.admin_fee = 30;

    let signature = parameters.sign_claim(&clp);
    let response = contract
        .claim(&parameters.alice, clp.clone(), signature)
        .await
        .unwrap();

    check_event(
        response,
        PinMinted {
            recipient: alice,
            pin_id: 1,
        },
    );

    let balance = contract.balance_of(alice).await.unwrap();
    assert_eq!(balance, 2);

    let pin_owner = contract.pin_owner(1).await.unwrap();
    assert_eq!(pin_owner, Some(alice));

    let total_minted = contract.total_minted().await.unwrap();
    assert_eq!(total_minted, 2);

    let total_minted_per_guild = contract.total_minted_per_guild(clp.guild_id).await.unwrap();
    assert_eq!(total_minted_per_guild, 2);

    let pin_id = contract
        .pin_id_by_address(alice, clp.guild_id, GuildAction::Joined)
        .await
        .unwrap();
    assert_eq!(pin_id, Some(0));

    let pin_id = contract
        .pin_id_by_user_id(clp.user_id, clp.guild_id, GuildAction::Joined)
        .await
        .unwrap();
    assert_eq!(pin_id, Some(0));

    let pin_id = contract
        .pin_id_by_address(alice, clp.guild_id, clp.action.clone())
        .await
        .unwrap();
    assert_eq!(pin_id, Some(1));

    let pin_id = contract
        .pin_id_by_user_id(clp.user_id, clp.guild_id, clp.action)
        .await
        .unwrap();
    assert_eq!(pin_id, Some(1));

    let base_balance = parameters
        .provider()
        .get_asset_balance(parameters.alice.address(), AssetId::BASE)
        .await
        .unwrap();
    assert_eq!(base_balance, genesis_balance - 2 * fee - clp.admin_fee);

    let base_balance = parameters
        .provider()
        .get_asset_balance(parameters.treasury.address(), AssetId::BASE)
        .await
        .unwrap();
    assert_eq!(base_balance, genesis_balance + 2 * fee);

    // let base_balance = parameters
    //     .provider()
    //     .get_asset_balance(parameters.charlie.address(), AssetId::BASE)
    //     .await
    //     .unwrap();
    // assert_eq!(base_balance, genesis_balance + clp.admin_fee);

    let pin_balance = parameters
        .provider()
        .get_asset_balance(parameters.alice.address(), contract.asset_id())
        .await
        .unwrap();
    assert_eq!(pin_balance, 2);

    let pin_balances = parameters
        .provider()
        .get_contract_balances(contract.bech_contract_id())
        .await
        .unwrap();
    assert_eq!(pin_balances.get(&contract.asset_id()), Some(&0));
}

#[tokio::test]
async fn token_of_owner_by_index_ok() {
    // Alice mints the first and third pins, bob the second
    let fee = 20;
    let genesis_balance = 100;
    let parameters = ParametersBuilder::new()
        .fee(fee)
        .genesis_balance(genesis_balance)
        .test()
        .await;
    let contract = GuildPinContract::init(&parameters).await;

    let alice: Address = parameters.alice.address().into();
    let bob: Address = parameters.bob.address().into();
    let clp = ClaimBuilder::new(alice, contract.contract_id())
        .guild_id(111)
        .build();

    let signature = parameters.sign_claim(&clp);
    contract
        .claim(&parameters.alice, clp, signature)
        .await
        .unwrap();

    let clp = ClaimBuilder::new(bob, contract.contract_id())
        .guild_id(555)
        .build();

    let signature = parameters.sign_claim(&clp);
    contract
        .claim(&parameters.bob, clp, signature)
        .await
        .unwrap();

    let clp = ClaimBuilder::new(alice, contract.contract_id())
        .guild_id(999)
        .build();

    let signature = parameters.sign_claim(&clp);
    contract
        .claim(&parameters.alice, clp, signature)
        .await
        .unwrap();

    let balance_alice = contract.balance_of(alice).await.unwrap();
    assert_eq!(balance_alice, 2);
    let balance_bob = contract.balance_of(bob).await.unwrap();
    assert_eq!(balance_bob, 1);
    let total_minted = contract.total_minted().await.unwrap();
    assert_eq!(total_minted, 3);

    assert_eq!(
        contract.token_of_owner_by_index(alice, 0).await.unwrap(),
        Some(0)
    );
    assert_eq!(
        contract.token_of_owner_by_index(alice, 1).await.unwrap(),
        Some(2)
    );
    assert!(contract
        .token_of_owner_by_index(alice, 2)
        .await
        .unwrap()
        .is_none());
    assert_eq!(
        contract.token_of_owner_by_index(bob, 0).await.unwrap(),
        Some(1)
    );
    assert!(contract
        .token_of_owner_by_index(bob, 1)
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn claim_uninitialized_fails() {
    let parameters = ParametersBuilder::new().test().await;
    let contract = GuildPinContract::deploy(&parameters).await;

    let alice: Address = parameters.alice.address().into();
    let clp = ClaimBuilder::new(alice, contract.contract_id()).build();

    let signature = parameters.sign_claim(&clp);
    let error = contract
        .claim(&parameters.alice, clp, signature)
        .await
        .unwrap_err();

    check_error(error, "NotInitialized");
}

#[tokio::test]
async fn claim_double_fails() {
    let parameters = ParametersBuilder::new().test().await;
    let contract = GuildPinContract::init(&parameters).await;

    let alice: Address = parameters.alice.address().into();
    let mut clp = ClaimBuilder::new(alice, contract.contract_id()).build();

    let signature = parameters.sign_claim(&clp);
    contract
        .claim(&parameters.alice, clp.clone(), signature)
        .await
        .unwrap();

    assert_eq!(contract.balance_of(alice).await.unwrap(), 1);

    // change address
    clp.recipient = parameters.bob.address().into();
    let signature = parameters.sign_claim(&clp);
    let error = contract
        .claim(&parameters.alice, clp.clone(), signature)
        .await
        .unwrap_err();

    check_error(error, "AlreadyClaimed");

    // change user_id
    let clp = ClaimBuilder::new(alice, contract.contract_id())
        .user_id(99999)
        .build();
    let signature = parameters.sign_claim(&clp);
    let error = contract
        .claim(&parameters.alice, clp, signature)
        .await
        .unwrap_err();

    check_error(error, "AlreadyClaimed");
}

#[tokio::test]
async fn claim_with_invalid_signature_fails() {
    let parameters = ParametersBuilder::new().test().await;
    let contract = GuildPinContract::init(&parameters).await;

    let alice: Address = parameters.alice.address().into();
    let clp = ClaimBuilder::new(alice, contract.contract_id()).build();
    let signature = parameters.sign_alt_claim(&clp);
    let error = contract
        .claim(&parameters.alice, clp, signature)
        .await
        .unwrap_err();

    check_error(error, "InvalidSignature");
}

#[tokio::test]
async fn claim_with_invalid_fee_fails() {
    let parameters = ParametersBuilder::new().test().await;
    let contract = GuildPinContract::init(&parameters).await;

    let alice: Address = parameters.alice.address().into();
    let clp = ClaimBuilder::new(alice, contract.contract_id()).build();
    let signature = parameters.sign_claim(&clp);
    let error = contract
        .unsafe_claim(&parameters.alice, clp.clone(), signature, 0, AssetId::BASE)
        .await
        .unwrap_err();

    check_error(error, "InsufficientAmount");

    let error = contract
        .unsafe_claim(&parameters.alice, clp, signature, 0, contract.asset_id())
        .await
        .unwrap_err();

    check_error(error, "InvalidAssetId");
}

#[tokio::test]
async fn claim_with_expired_signature_fails() {
    let parameters = ParametersBuilder::new().test().await;
    let contract = GuildPinContract::init(&parameters).await;

    let alice: Address = parameters.alice.address().into();
    let clp = ClaimBuilder::new(alice, contract.contract_id())
        .signed_at(parameters.timestamp().await - 4000)
        .build();
    let signature = parameters.sign_claim(&clp);
    let error = contract
        .claim(&parameters.alice, clp.clone(), signature)
        .await
        .unwrap_err();

    check_error(error, "ExpiredSignature");
}

#[tokio::test]
async fn claim_with_invalid_contract_id_fails() {
    let parameters = ParametersBuilder::new().test().await;
    let contract = GuildPinContract::init(&parameters).await;

    let alice: Address = parameters.alice.address().into();
    let clp = ClaimBuilder::new(alice, ContractId::zeroed()).build();
    let signature = parameters.sign_claim(&clp);
    let error = contract
        .claim(&parameters.alice, clp.clone(), signature)
        .await
        .unwrap_err();

    check_error(error, "InvalidContractId");
}
