pub mod address_book;
pub mod alert;
pub mod block_scanner;
pub mod dex;
pub mod helpers;
pub mod mempool;
pub mod uni;

use std::sync::Arc;

use address_book::*;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use helpers::address;

use crate::dex::Dex;
use crate::helpers::setup_signer;

pub struct Config {
    #[allow(dead_code)]
    pub http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    #[allow(dead_code)]
    pub wss: Arc<Provider<Ws>>,
}

impl Config {
    pub async fn new() -> Self {
        let network = std::env::var("NETWORK_RPC").expect("missing NETWORK_RPC");
        let provider: Provider<Http> = Provider::<Http>::try_from(network).unwrap();
        let middleware = Arc::new(setup_signer(provider.clone()).await);

        let ws_network = std::env::var("NETWORK_WSS").expect("missing NETWORK_WSS");
        let ws_provider: Provider<Ws> = Provider::<Ws>::connect(ws_network).await.unwrap();
        Self {
            http: middleware,
            wss: Arc::new(ws_provider),
        }
    }

    pub async fn create_dex(&self, factory: Address, router: Address) -> Dex {
        Dex::new(self.http.clone(), factory, router)
    }
}

/// Run the strategy here.
pub async fn run() {
    let config = Config::new().await;

    // Example of how to interact with a contract.
    let spooky_factory = address(SPOOKY_SWAP_FACTORY);
    let spooky_router = address(SPOOKY_SWAP_ROUTER);
    let dex = config.create_dex(spooky_factory, spooky_router).await;
    dex.get_pairs().await;

    // Thread for checking what block we're on.
    tokio::spawn(async move {
        block_scanner::loop_blocks(Arc::clone(&config.http)).await;
    });

    // Main loop to monitor the mempool.
    mempool::loop_mempool(Arc::clone(&config.wss)).await;
}
