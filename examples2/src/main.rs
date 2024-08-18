use bytes::Bytes;
use fastcrypto::traits::EncodeDecodeBase64;
use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::bls12381;
use fastcrypto::bls12381::min_sig::{
    BLS12381AggregateSignature, BLS12381AggregateSignatureAsBytes, BLS12381KeyPair,
    BLS12381PrivateKey, BLS12381PublicKey, BLS12381Signature,
};
use fastcrypto::traits::KeyPair;
use narwhal_config::{Epoch, WorkerCache, WorkerIndex, WorkerId, WorkerInfo};
use narwhal_types::{TransactionProto, TransactionsClient};
use std::str::FromStr;
use std::time::Duration;
use sui_types::{crypto::ToFromBytes, sui_system_state::SuiSystemStateTrait};
use tokio::time::{interval, sleep};
use sui_swarm_config::network_config_builder::ConfigBuilder;
use std::sync::Arc;
use sui_core::authority::test_authority_builder::TestAuthorityBuilder;
use sui_types::sui_system_state::epoch_start_sui_system_state::EpochStartSystemStateTrait;
use base64::{Engine as _, engine::general_purpose};
use mysten_network::Multiaddr;

use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let configs = ConfigBuilder::new_with_temp_dir()
        .committee_size(1.try_into().unwrap())
        .build();

    let config = &configs.validator_configs()[0];
    let consensus_config = config.consensus_config().unwrap();
    let secret = Arc::pin(config.protocol_key_pair().copy());
    let genesis = config.genesis().unwrap();

    let state = TestAuthorityBuilder::new()
        .with_genesis_and_keypair(genesis, &secret)
        .build()
        .await;

    let system_state = state
        .get_sui_system_state_object_for_testing()
        .expect("Reading Sui system state object cannot fail")
        .into_epoch_start_state();

    let transactions_addr = &config.consensus_config.as_ref().unwrap().address;
    let narwhal_committee = system_state.get_narwhal_committee();
    let mut worker_cache = system_state.get_narwhal_worker_cache(transactions_addr);

    let worker_pubkey = "omHynOXcJkn6xd9srFQu4P4gsLTdDqMif6YFQPA/fnYICGMwvYs5JOvUjUnMSoBpFbDX/Mk8HRWMT8k6QXJU7ZFVnZAmqWMbu86zqWBQVv5mIi47aWFNEaebQYh+ke+9";
    let pub_key_bytes = general_purpose::STANDARD.decode(worker_pubkey).unwrap();
    let worker_publickey = BLS12381PublicKey::from_bytes(&pub_key_bytes).unwrap();

    let base64_string = "tVbCtEcFJg8uI2uYr7hKNbuVu+brORJT9oQndNaPWtQ=";
    let key_bytes = general_purpose::STANDARD.decode(base64_string).unwrap();

    let mut worker_index_inner = BTreeMap::new();
    worker_index_inner.insert(
        0_u32, 
        WorkerInfo {
            name: Ed25519PublicKey::from_bytes(&key_bytes).unwrap(), 
            transactions: Multiaddr::from_str("/ip4/127.0.0.1/tcp/0/http").unwrap(),
            worker_address: Multiaddr::from_str("/ip4/127.0.0.1/udp/10").unwrap()
        }
    );
    let worker_index = WorkerIndex(worker_index_inner);
    

    worker_cache.workers.insert(worker_publickey.clone(), worker_index);
    let workers = worker_cache.workers.keys().collect::<Vec<_>>();
    println!("{workers:#?}");

    // let worker0_key = workers[0].clone();    
    
    println!("Base64 string: {}", base64_string);
    println!("Decoded bytes: {:?}", key_bytes);

    // let name = BLS12381KeyPair::decode_base64(base64_string).unwrap();
    // // let name = bls12381::min_sig::BLS12381PublicKey::from_bytes(&key_bytes).unwrap();
    // // let worker_cache = WorkerCache::default();
    let epoch = Epoch::default();
    // let target = "127.0.0.1:0".to_string(); // Replace with your Narwhal node's address

    // send_transactions(&name.public(), worker_cache, epoch).await;
    send_transactions(&worker_publickey, worker_cache, epoch).await;
}

async fn send_transactions(
    name: &bls12381::min_sig::BLS12381PublicKey,
    worker_cache: WorkerCache,
    epoch: Epoch,
    // target: String,
) {
    let target = worker_cache
        .worker(name, /* id */ &0)
        .expect("Our key or worker id is not in the worker cache")
        .transactions;
    let config = mysten_network::config::Config::new();
    let channel = config.connect_lazy(&target).unwrap();
    let mut client = TransactionsClient::new(channel);

    // Make a transaction to submit forever.
    let tx = TransactionProto {
        transactions: vec![Bytes::from(epoch.to_be_bytes().to_vec())],
    };

    // Repeatedly send transactions.
    let interval = interval(Duration::from_millis(1));
    tokio::pin!(interval);

    let mut succeeded_once = false;
    loop {
        tokio::select! {
            _ = interval.tick() => {
                // Send a transaction.
                let result = client.submit_transaction(tx.clone()).await;
                if result.is_ok() {
                    succeeded_once = true;
                    println!("Transaction sent successfully");
                } else {
                    println!("Failed to send transaction: {:?}", result.err());
                }
            }
        }

        // Break the loop after sending transactions for 5 seconds
        sleep(Duration::from_secs(5)).await;
        break;
    }

    assert!(succeeded_once);
}