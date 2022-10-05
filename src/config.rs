use std::sync::Arc;

use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;

use crate::addresses::setup_signer;

pub struct Config(Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>);

impl Config {
    pub async fn new() -> Self {
        let network = std::env::var("NETWORK_RPC").expect("missing NETWORK_RPC");
        let provider: Provider<Http> = Provider::<Http>::try_from(network).unwrap();
        let middleware = Arc::new(setup_signer(provider.clone()).await);
        Self(middleware)
    }

    // pub async fn create_contract(&self) -> ContractStruct {
    //     ContractStruct::new(&self.middleware)
    // }
}
