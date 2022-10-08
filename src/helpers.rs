use ethers::prelude::{k256::ecdsa::SigningKey, *};

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

/// Creates a binding for an ABI.
/// Example: bind("Example", "src/abi/example.json");
pub fn bind(name: &str, abi: &str) {
    let name: String = format!("b_{}", name);
    let bindings = Abigen::new(&name, abi).unwrap().generate().unwrap();
    let path: String = format!("src/bindings/{}.rs", name);
    match std::fs::File::create(path.clone()) {
        Ok(_) => {}
        Err(_) => {}
    }
    bindings.write_to_file(&path).unwrap();
}
