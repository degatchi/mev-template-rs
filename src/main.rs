use mev_template::{
    addresses::{address, SPOOKY_SWAP_FACTORY, SPOOKY_SWAP_ROUTER},
    config::Config,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = Config::new().await;

    let spooky_factory = address(SPOOKY_SWAP_FACTORY);
    let spooky_router = address(SPOOKY_SWAP_ROUTER);
    
    let dex = config.create_dex(spooky_factory, spooky_router).await;
    dex.get_pairs().await;
}
