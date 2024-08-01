use beerus::gen::{
    Address, BlockId, BlockTag, BroadcastedDeclareTxn, BroadcastedTxn, Felt,
    Rpc,
};
use common::{
    api::{
        setup_client_with_mock_starknet_node,
        StarknetMatcher::{
            ChainId, ClassError, ClassSuccess, EstimateFee, Nonce, SpecVersion,
        },
    },
    constants::declare_transaction_v2,
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
    let declare_transaction = declare_transaction_v2();
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
