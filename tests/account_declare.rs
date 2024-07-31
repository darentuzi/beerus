use beerus::gen::{
    Address, BlockId, BlockTag, BroadcastedDeclareTxn, BroadcastedDeclareTxnV2,
    BroadcastedDeclareTxnV2Type, BroadcastedDeclareTxnV2Version,
    BroadcastedTxn, ContractClass, ContractClassEntryPointsByType, Felt, Rpc,
    SierraEntryPoint,
};
use common::api::{
    setup_client_with_mock_starknet_node,
    StarknetMatcher::{
        ChainId, ClassError, ClassSuccess, EstimateFee, Nonce, SpecVersion,
    },
};

mod common;

#[tokio::test]
async fn chain_id_test() {
    let (client, _starknet_node) =
        setup_client_with_mock_starknet_node(vec![ChainId]).await;
    let result = client.chainId().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn chain_id_nonce() {
    let (client, _starknet_node) =
        setup_client_with_mock_starknet_node(vec![ChainId, Nonce]).await;
    assert!(client.chainId().await.is_ok());
    assert!(client
        .getNonce(
            BlockId::BlockTag(BlockTag::Latest),
            Address(Felt::try_new("0x0").unwrap())
        )
        .await
        .is_ok())
}

#[tokio::test]
async fn chain_id_called_twice() {
    let (client, _starknet_node) =
        setup_client_with_mock_starknet_node(vec![ChainId, ChainId]).await;
    assert!(client.chainId().await.is_ok());
    assert!(client.chainId().await.is_ok());
}

#[tokio::test]
async fn get_class_error() {
    let (client, _starknet_node) =
        setup_client_with_mock_starknet_node(vec![ClassError]).await;
    assert!(client
        .getClass(
            BlockId::BlockTag(BlockTag::Latest),
            Felt::try_new("0x0").unwrap()
        )
        .await
        .is_err());
}

#[tokio::test]
async fn get_class_success() {
    let (client, _starknet_node) =
        setup_client_with_mock_starknet_node(vec![ClassSuccess]).await;
    assert!(client
        .getClass(
            BlockId::BlockTag(BlockTag::Latest),
            Felt::try_new("0x0").unwrap()
        )
        .await
        .is_ok());
}

#[tokio::test]
async fn spec_version_estimate_fee() {
    let declare_transaction = BroadcastedDeclareTxnV2 {
        compiled_class_hash: Felt::try_new("0x0").unwrap(),
        contract_class: ContractClass {
            sierra_program: vec![Felt::try_new("0x1").unwrap()],
            contract_class_version: "0.1.0".to_string(),
            entry_points_by_type: ContractClassEntryPointsByType {
                constructor: vec![SierraEntryPoint {
                    selector: Felt::try_new("0x2").unwrap(),
                    function_idx: 2,
                }],
                external: vec![
                    SierraEntryPoint {
                        selector: Felt::try_new("0x3").unwrap(),
                        function_idx: 3,
                    },
                    SierraEntryPoint {
                        selector: Felt::try_new("0x4").unwrap(),
                        function_idx: 4,
                    },
                ],
                l1_handler: vec![],
            },
            abi: Some("some_abi".to_string()),
        },
        max_fee: Felt::try_new("0x0").unwrap(),
        nonce: Felt::try_new("0x0").unwrap(),
        r#type: BroadcastedDeclareTxnV2Type::Declare,
        signature: vec![Felt::try_new("0x5").unwrap()],
        sender_address: Address(Felt::try_new("0x6").unwrap()),
        version: BroadcastedDeclareTxnV2Version::V0x2,
    };
    let (client, _starknet_node) =
        setup_client_with_mock_starknet_node(vec![SpecVersion, EstimateFee])
            .await;
    assert!(client.specVersion().await.is_ok());
    let res = client
        .estimateFee(
            vec![BroadcastedTxn::BroadcastedDeclareTxn(
                BroadcastedDeclareTxn::BroadcastedDeclareTxnV2(
                    declare_transaction,
                ),
            )],
            vec![],
            BlockId::BlockTag(BlockTag::Latest),
        )
        .await;
    assert!(res.is_ok());
}
