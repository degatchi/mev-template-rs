use std::sync::Arc;

use ethers::prelude::{abi::AbiDecode, k256::ecdsa::SigningKey, *};

use crate::address_book::{UniV2Factory, UniV2Router, UniV2RouterCalls};

#[allow(dead_code)]
pub struct Dex {
    factory_address: Address,
    router_address: Address,
    factory: UniV2Factory<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    router: UniV2Router<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl Dex {
    pub fn new(
        middleware: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        factory_address: Address,
        router_address: Address,
    ) -> Self {
        let factory = UniV2Factory::new(factory_address, Arc::clone(&middleware));
        let router = UniV2Router::new(router_address, Arc::clone(&middleware));
        Self {
            factory_address,
            router_address,
            factory,
            router,
        }
    }

    /// A quick way to decode tx hex data.
    pub async fn decode_router_tx_data(&self, tx_data: String) {
        let calldata: Bytes = tx_data.parse().unwrap();
        let decoded = UniV2RouterCalls::decode(&calldata).unwrap();
        println!("Decoded dex tx: {:?}", decoded);
    }

    /// Attempts to retrieve the total pairs created from the dex's factory.
    pub async fn get_pairs(&self) {
        println!("Calling allPairsLength from {}", self.factory_address);
        match self.factory.all_pairs_length().call().await {
            Ok(result) => {
                println!("   ~ [PASS] Total pairs: {:?}", result)
            }
            Err(e) => {
                println!("   ~ [FAIL] Total pairs: {:?}", e)
            }
        }
    }

    /// Streams the "PairCreated" event from the `factory_address`.
    pub async fn stream_pairs_created(&self, ws: &Provider<Ws>) {
        let filter = Filter::new()
            .address(self.factory_address)
            .event("PairCreated");

        let mut stream: SubscriptionStream<Ws, Log> = ws.subscribe_logs(&filter).await.unwrap();

        println!("Listening for PairCreated events, from {}", self.factory_address);
        while let Some(log) = stream.next().await {
            println!(
                "   ~ [FOUND] Hash {:?}\nLog: {:?}",
                log.transaction_hash,
                log.data,
                // PsNewSale::decode(log.data)
            );
        }
    }
}
