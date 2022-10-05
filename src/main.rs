use mev_template::config::Config;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = Config::new().await;

}
