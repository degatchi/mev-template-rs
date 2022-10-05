use std::sync::Arc;

use ethers::prelude::{abi::AbiDecode, k256::ecdsa::SigningKey, *};

use crate::addresses::{UniV2Factory, UniV2Router, UniV2RouterCalls};

pub struct Dex {
    #[allow(dead_code)]
    factory: UniV2Factory<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    #[allow(dead_code)]
    router: UniV2Router<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl Dex {
    pub fn new(
        middleware: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        factory: Address,
        router: Address,
    ) -> Self {
        let factory = UniV2Factory::new(factory, Arc::clone(&middleware));
        let router = UniV2Router::new(router, Arc::clone(&middleware));
        Self { factory, router }
    }

    /// A quick way to decode tx hex data.
    pub async fn decode_router_tx_data(&self, tx_data: String) {
        let calldata: Bytes = tx_data.parse().unwrap();
        let decoded = UniV2RouterCalls::decode(&calldata).unwrap();
        println!("Decoded dex tx: {:?}", decoded);
    }

    /// Attempts to retrieve the total pairs created from the dex's factory.
    pub async fn get_pairs(&self) {
        match self.factory.all_pairs_length().call().await {
            Ok(result) => {
                println!("[PASS] Total pairs: {:?}", result)
            }
            Err(e) => {
                println!("[FAIL] Total pairs: {:?}", e)
            }
        }
    }
}
