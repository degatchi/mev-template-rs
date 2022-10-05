use ethers::prelude::{k256::ecdsa::SigningKey, *};

pub const SMART_CONTRACT_ADDRESS: &str = "";

abigen!(SmartContract, "src/abi/LpPair.json");

/// Converts &str to Address.
pub fn address(address: &str) -> Address {
    address.parse::<Address>().unwrap()
}

/// Converts normal input into 1e18.
pub fn to_1e18(input: u64) -> U256 {
    let ether: U256 = U256::exp10(18);
    let parsed: U256 = input.into();
    parsed * ether
}

/// Sets up middleware w/ our private key env var.
pub async fn setup_signer(
    provider: Provider<Http>,
) -> SignerMiddleware<Provider<Http>, Wallet<SigningKey>> {
    let chain_id = provider
        .get_chainid()
        .await
        .expect("Failed to get chain id.");

    let priv_key = std::env::var("PRIVATE_KEY").expect("missing PRIVATE_KEY");

    let wallet = priv_key
        .parse::<LocalWallet>()
        .expect("Failed to parse wallet")
        .with_chain_id(chain_id.as_u64());

    SignerMiddleware::new(provider, wallet)
}
