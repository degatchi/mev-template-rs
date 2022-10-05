use std::sync::Arc;

use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;

use crate::addresses::setup_signer;
use crate::dex::Dex;

pub struct Config {
    #[allow(dead_code)]
    http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    #[allow(dead_code)]
    wss: Arc<Provider<Ws>>,
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
