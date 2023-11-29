use crate::contract::{ClaimParameters, GuildAction, GuildPinContract, PinMinted};
use crate::parameters::ParametersBuilder;
use crate::{check_error, check_event};
use fuels::types::{Address, AssetId, Bits256, Identity, SizedAsciiString};

const CID64: &str = "abcdefghijklmnopqrstuvxyzabcdefghijklmnopqrstuvxyzabcdefghijklmn";

#[tokio::test]
async fn claim_successful() {
    let fee = 20;
    let genesis_balance = 100;
    let parameters = ParametersBuilder::new()
        .with_fee(fee)
        .with_genesis_balance(genesis_balance)
        .build()
        .await;
    let contract = GuildPinContract::init(&parameters).await;

    let alice: Address = parameters.alice.address().into();
    let guild_id = 1234;
    let user_id = 10;
    let action = GuildAction::Joined;
    let mut claim_params = ClaimParameters {
        recipient: alice,
        action: action.clone(),
        user_id,
        guild_id,
        guild_name: SizedAsciiString::new_with_right_whitespace_padding(
            "MyAwesomeGuild".to_string(),
        )
        .unwrap(),
        created_at: 1000,
        signed_at: 1000000000000,
        cid: SizedAsciiString::new_with_right_whitespace_padding(CID64.to_string()).unwrap(),
        admin_treasury: Identity::ContractId(contract.contract_id()),
        admin_fee: 0,
        contract_id: contract.contract_id(),
    };

    let signature = parameters.sign_claim(&claim_params);
    let response = contract
        .claim(&parameters.alice, claim_params, signature)
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

    let total_minted_per_guild = contract.total_minted_per_guild(guild_id).await.unwrap();
    assert_eq!(total_minted_per_guild, 1);

    let pin_id = contract
        .pin_id_by_address(alice, guild_id, action.clone())
        .await
        .unwrap();
    assert_eq!(pin_id, Some(0));

    let pin_id = contract
        .pin_id_by_user_id(user_id, guild_id, action.clone())
        .await
        .unwrap();
    assert_eq!(pin_id, Some(0));

    let pin_id = contract.pin_id_by_address(alice, 11, action).await.unwrap();
    assert_eq!(pin_id, None);

    let pin_id = contract
        .pin_id_by_user_id(user_id, guild_id, GuildAction::Owner)
        .await
        .unwrap();
    assert_eq!(pin_id, None);

    let base_balance = parameters
        .provider
        .get_asset_balance(parameters.alice.address(), AssetId::BASE)
        .await
        .unwrap();
    assert_eq!(base_balance, genesis_balance - fee);

    let base_balance = parameters
        .provider
        .get_asset_balance(parameters.treasury.address().into(), AssetId::BASE)
        .await
        .unwrap();
    assert_eq!(base_balance, genesis_balance + fee);

    let pin_balance = parameters
        .provider
        .get_asset_balance(parameters.alice.address(), contract.asset_id())
        .await
        .unwrap();
    assert_eq!(pin_balance, 0);

    let pin_balances = parameters
        .provider
        .get_contract_balances(contract.bech_contract_id())
        .await
        .unwrap();
    assert_eq!(pin_balances.get(&contract.asset_id()), Some(&1));
}
