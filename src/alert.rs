use std::collections::HashMap;

/// Alerts discord channel, via webhook, we found an opportunity.
pub async fn alert(msg: &str, block: &u64) {
    let msg = format!(
        "-----------------------------\n🔍 Block: {:?}\n-----------------------------\n{}",
        block, msg
    );

    let max_length = 1900.min(msg.len());
    let message = msg[..max_length].to_string();
    let mut map = HashMap::new();
    map.insert("content", message.to_string());

    let webhook = std::env::var("DISCORD_WEBHOOK").expect("missing DISCORD_WEBHOOK");
    let client = reqwest::Client::new();
    let res = client.post(webhook.to_string()).json(&map).send().await;

    match res {
        Ok(_) => {}
        Err(err) => {
            println!("Could not send alert to discord, err: {}", err);
            println!("Message: {}", message);
        }
    }
}
