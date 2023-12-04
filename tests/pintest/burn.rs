use crate::{check_error, check_event};
use fuels::types::{Address, AssetId};
use guild_pin_contract::contract::{GuildAction, GuildPinContract, PinBurned, PinMinted};
use guild_pin_contract::parameters::ParametersBuilder;
use guild_pin_contract::utils::ClaimBuilder;

#[tokio::test]
async fn burn_successful() {
    let parameters = ParametersBuilder::new().test().await;
    let contract = GuildPinContract::init(&parameters).await;

    let alice: Address = parameters.alice.address().into();
    let clp = ClaimBuilder::new(alice, contract.contract_id()).build();

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

    assert_eq!(contract.total_supply().await.unwrap(), 1);

    let response = contract.burn(&parameters.alice, 0).await.unwrap();
    check_event(
        response,
        PinBurned {
            pin_owner: alice,
            pin_id: 0,
        },
    );

    assert_eq!(contract.total_supply().await.unwrap(), 0);
    assert_eq!(contract.balance_of(alice).await.unwrap(), 0);
    assert!(contract.pin_owner(0).await.unwrap().is_none());
    assert_eq!(contract.total_minted().await.unwrap(), 1);
    let total_minted_per_guild = contract.total_minted_per_guild(clp.guild_id).await.unwrap();
    assert_eq!(total_minted_per_guild, 1);
    let pin_id = contract
        .pin_id_by_address(alice, clp.guild_id, clp.action.clone())
        .await
        .unwrap();
    assert_eq!(pin_id, None);

    let pin_id = contract
        .pin_id_by_user_id(clp.user_id, clp.guild_id, clp.action.clone())
        .await
        .unwrap();
    assert_eq!(pin_id, None);

    // claim again
    let clp = ClaimBuilder::new(alice, contract.contract_id()).build();

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

    assert_eq!(contract.total_supply().await.unwrap(), 1);
    assert_eq!(contract.balance_of(alice).await.unwrap(), 1);
    assert!(contract.pin_owner(0).await.unwrap().is_none());
    assert_eq!(contract.pin_owner(1).await.unwrap(), Some(alice));
    assert_eq!(contract.total_minted().await.unwrap(), 2);
    let total_minted_per_guild = contract.total_minted_per_guild(clp.guild_id).await.unwrap();
    assert_eq!(total_minted_per_guild, 2);
    let pin_id = contract
        .pin_id_by_address(alice, clp.guild_id, clp.action.clone())
        .await
        .unwrap();
    assert_eq!(pin_id, Some(1));

    let pin_id = contract
        .pin_id_by_user_id(clp.user_id, clp.guild_id, clp.action.clone())
        .await
        .unwrap();
    assert_eq!(pin_id, Some(1));
}

#[tokio::test]
async fn burn_with_not_owner_fails() {
    let parameters = ParametersBuilder::new().test().await;
    let contract = GuildPinContract::init(&parameters).await;

    let alice: Address = parameters.alice.address().into();
    let clp = ClaimBuilder::new(alice, contract.contract_id()).build();

    let signature = parameters.sign_claim(&clp);
    contract
        .claim(&parameters.alice, clp.clone(), signature)
        .await
        .unwrap();

    let error = contract.burn(&parameters.bob, 0).await.unwrap_err();
    check_error(error, "NotPinOwner");
}

#[tokio::test]
async fn burn_double_fails() {
    let parameters = ParametersBuilder::new().test().await;
    let contract = GuildPinContract::init(&parameters).await;

    let alice: Address = parameters.alice.address().into();
    let clp = ClaimBuilder::new(alice, contract.contract_id()).build();

    let signature = parameters.sign_claim(&clp);
    contract
        .claim(&parameters.alice, clp.clone(), signature)
        .await
        .unwrap();

    contract.burn(&parameters.alice, 0).await.unwrap();
    let error = contract.burn(&parameters.alice, 0).await.unwrap_err();
    check_error(error, "AlreadyBurned");
}

#[tokio::test]
async fn burn_non_existent_fails() {
    let parameters = ParametersBuilder::new().test().await;
    let contract = GuildPinContract::init(&parameters).await;

    let alice: Address = parameters.alice.address().into();
    let clp = ClaimBuilder::new(alice, contract.contract_id()).build();

    let signature = parameters.sign_claim(&clp);
    contract
        .claim(&parameters.alice, clp.clone(), signature)
        .await
        .unwrap();

    let error = contract.burn(&parameters.alice, 99).await.unwrap_err();
    check_error(error, "PinIdDoesNotExist");
}

#[tokio::test]
async fn flow_successful() {
    // alice, bob, charlie all claim some pins and in the end we check the storage
    let genesis_balance = 100_000;
    let parameters = ParametersBuilder::new()
        .genesis_balance(genesis_balance)
        .test()
        .await;
    let contract = GuildPinContract::init(&parameters).await;

    let admin_treasury = parameters.owner_id();
    let admin_fee = 5;

    let alice: Address = parameters.alice.address().into();
    let bob: Address = parameters.bob.address().into();
    let charlie: Address = parameters.charlie.address().into();

    let alice_id: u64 = 111;
    let bob_id: u64 = 222;
    let charlie_id: u64 = 333;

    let guild_0: u64 = 1234;
    let guild_1: u64 = 5678;
    let guild_2: u64 = 9999;

    // everybody joins guild_0 where
    // alice is admin as well
    // bob is owner as well
    let claim = ClaimBuilder::new(alice, contract.contract_id())
        .user_id(alice_id)
        .guild_id(guild_0)
        .admin_treasury(admin_treasury.clone())
        .admin_fee(admin_fee)
        .chain_id(u64::MAX)
        .build();
    assert_eq!(claim.chain_id, u64::MAX);
    let signature = parameters.sign_claim(&claim);
    contract
        .claim(&parameters.alice, claim, signature)
        .await
        .unwrap();

    let claim = ClaimBuilder::new(bob, contract.contract_id())
        .user_id(bob_id)
        .guild_id(guild_0)
        .admin_treasury(admin_treasury.clone())
        .admin_fee(admin_fee)
        .build();
    let signature = parameters.sign_claim(&claim);
    contract
        .claim(&parameters.bob, claim, signature)
        .await
        .unwrap();

    let claim = ClaimBuilder::new(charlie, contract.contract_id())
        .user_id(charlie_id)
        .guild_id(guild_0)
        .admin_treasury(admin_treasury.clone())
        .admin_fee(admin_fee)
        .build();
    let signature = parameters.sign_claim(&claim);
    contract
        .claim(&parameters.charlie, claim, signature)
        .await
        .unwrap();

    let claim = ClaimBuilder::new(alice, contract.contract_id())
        .user_id(alice_id)
        .guild_id(guild_0)
        .admin_treasury(admin_treasury.clone())
        .admin_fee(admin_fee)
        .action(GuildAction::Admin)
        .build();
    let signature = parameters.sign_claim(&claim);
    contract
        .claim(&parameters.alice, claim, signature)
        .await
        .unwrap();

    let claim = ClaimBuilder::new(bob, contract.contract_id())
        .user_id(bob_id)
        .guild_id(guild_0)
        .admin_treasury(admin_treasury.clone())
        .admin_fee(admin_fee)
        .action(GuildAction::Owner)
        .build();
    let signature = parameters.sign_claim(&claim);
    contract
        .claim(&parameters.bob, claim, signature)
        .await
        .unwrap();

    // alice and bob join guild_1
    let claim = ClaimBuilder::new(alice, contract.contract_id())
        .user_id(alice_id)
        .guild_id(guild_1)
        .admin_treasury(admin_treasury.clone())
        .admin_fee(admin_fee)
        .build();
    let signature = parameters.sign_claim(&claim);
    contract
        .claim(&parameters.alice, claim, signature)
        .await
        .unwrap();

    let claim = ClaimBuilder::new(bob, contract.contract_id())
        .user_id(bob_id)
        .guild_id(guild_1)
        .admin_treasury(admin_treasury.clone())
        .admin_fee(admin_fee)
        .build();
    let signature = parameters.sign_claim(&claim);
    contract
        .claim(&parameters.bob, claim, signature)
        .await
        .unwrap();

    // charlie joins guild_2 that doesn't have an admin treasury
    let claim = ClaimBuilder::new(charlie, contract.contract_id())
        .user_id(charlie_id)
        .guild_id(guild_2)
        .build();
    let signature = parameters.sign_claim(&claim);
    contract
        .claim(&parameters.charlie, claim, signature)
        .await
        .unwrap();

    // check storage stuff
    assert_eq!(contract.total_supply().await.unwrap(), 8);
    assert_eq!(contract.total_minted_per_guild(guild_0).await.unwrap(), 5);
    assert_eq!(contract.total_minted_per_guild(guild_1).await.unwrap(), 2);
    assert_eq!(contract.total_minted_per_guild(guild_2).await.unwrap(), 1);
    assert_eq!(contract.balance_of(alice).await.unwrap(), 3);
    assert_eq!(contract.balance_of(bob).await.unwrap(), 3);
    assert_eq!(contract.balance_of(charlie).await.unwrap(), 2);
    assert_eq!(contract.pin_owner(0).await.unwrap(), Some(alice));
    assert_eq!(contract.pin_owner(1).await.unwrap(), Some(bob));
    assert_eq!(contract.pin_owner(2).await.unwrap(), Some(charlie));
    assert_eq!(contract.pin_owner(3).await.unwrap(), Some(alice));
    assert_eq!(contract.pin_owner(4).await.unwrap(), Some(bob));
    assert_eq!(contract.pin_owner(5).await.unwrap(), Some(alice));
    assert_eq!(contract.pin_owner(6).await.unwrap(), Some(bob));
    assert_eq!(contract.pin_owner(7).await.unwrap(), Some(charlie));
    let pin_id = contract
        .pin_id_by_user_id(alice_id, guild_0, GuildAction::Joined)
        .await
        .unwrap();
    assert_eq!(pin_id, Some(0));
    let pin_id = contract
        .pin_id_by_user_id(alice_id, guild_0, GuildAction::Admin)
        .await
        .unwrap();
    assert_eq!(pin_id, Some(3));
    let pin_id = contract
        .pin_id_by_user_id(bob_id, guild_0, GuildAction::Owner)
        .await
        .unwrap();
    assert_eq!(pin_id, Some(4));
    let pin_id = contract
        .pin_id_by_address(charlie, guild_2, GuildAction::Joined)
        .await
        .unwrap();
    assert_eq!(pin_id, Some(7));

    // admin_treasury = owner
    let admin_treasury_balance = parameters
        .provider()
        .get_asset_balance(parameters.owner.address(), AssetId::BASE)
        .await
        .unwrap();
    assert_eq!(admin_treasury_balance, genesis_balance + 7 * admin_fee); // charlie didn't pay admin fee
    let treasury_balance = parameters
        .provider()
        .get_asset_balance(parameters.treasury.address(), AssetId::BASE)
        .await
        .unwrap();
    assert_eq!(treasury_balance, genesis_balance + 8 * parameters.fee);

    // alice and bob burn their joined membersips in guild_0
    contract.burn(&parameters.alice, 0).await.unwrap();
    contract.burn(&parameters.bob, 1).await.unwrap();

    assert_eq!(contract.total_supply().await.unwrap(), 6);
    assert_eq!(contract.total_minted_per_guild(guild_0).await.unwrap(), 5);
    let pin_id = contract
        .pin_id_by_address(alice, guild_0, GuildAction::Joined)
        .await
        .unwrap();
    assert_eq!(pin_id, None);
    let pin_id = contract
        .pin_id_by_address(bob, guild_0, GuildAction::Joined)
        .await
        .unwrap();
    assert_eq!(pin_id, None);
    assert!(contract.pin_owner(0).await.unwrap().is_none());
    assert!(contract.pin_owner(1).await.unwrap().is_none());
    // alice becomes owner of guild_0, so she reclaims the joined pin and the owner pin as well
    let claim = ClaimBuilder::new(alice, contract.contract_id())
        .user_id(alice_id)
        .guild_id(guild_0)
        .admin_treasury(admin_treasury.clone())
        .admin_fee(admin_fee)
        .build();
    let signature = parameters.sign_claim(&claim);
    contract
        .claim(&parameters.alice, claim, signature)
        .await
        .unwrap();

    let claim = ClaimBuilder::new(alice, contract.contract_id())
        .user_id(alice_id)
        .guild_id(guild_0)
        .admin_treasury(admin_treasury.clone())
        .admin_fee(admin_fee)
        .action(GuildAction::Owner)
        .build();
    let signature = parameters.sign_claim(&claim);
    contract
        .claim(&parameters.alice, claim, signature)
        .await
        .unwrap();

    assert_eq!(contract.total_supply().await.unwrap(), 8);
    assert_eq!(contract.total_minted().await.unwrap(), 10);
    assert_eq!(contract.total_minted_per_guild(guild_0).await.unwrap(), 7);
    let pin_id = contract
        .pin_id_by_user_id(alice_id, guild_0, GuildAction::Joined)
        .await
        .unwrap();
    assert_eq!(pin_id, Some(8));
    let pin_id = contract
        .pin_id_by_user_id(alice_id, guild_0, GuildAction::Owner)
        .await
        .unwrap();
    assert_eq!(pin_id, Some(9));
    let pin_id = contract
        .pin_id_by_user_id(alice_id, guild_0, GuildAction::Admin)
        .await
        .unwrap();
    assert_eq!(pin_id, Some(3));
}
