use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use reqwest::Client;
use reqwest::header::{CONTENT_TYPE, CONTENT_LENGTH, ACCEPT};
use reqwest::header::{HeaderMap, USER_AGENT, HeaderValue, AUTHORIZATION};
use serde::{Serialize, Deserialize};

pub const DISCORD_API: &'static str = "https://discord.com/api/v9";
pub const USER_AGENT_VAL: &'static str = "Strife (https://github.com/Citrus-Studios, 0.0.1)";

#[tokio::test]
async fn test_api_test_fn() {
    api_test().await;
}

// Bot Gateway JSON
#[derive(Debug, Serialize, Deserialize)]
struct BotGateway {
    url: String,
    shards: i32,
    session_start_limit: SessionStartLimit,
}

#[derive(Debug, Serialize, Deserialize)]
struct SessionStartLimit {
    total: i32,
    remaining: i32,
    reset_after: i32,
    max_concurrency: i32,
}

// Early testing of the discord API
async fn api_test() {

    let bot_token = "Bot OTQ2NDc5Mjg3NzM1ODQwNzk4.YhfThg.2PHNRoLrczYKHYJzdaTK3g82WNs";

    let client = Arc::new(Client::new());
    let mut headers = HeaderMap::with_capacity(5);
    // Set the user agent header
    headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VAL));
    // Set the authorization header
    headers.insert(AUTHORIZATION, HeaderValue::from_str(bot_token).unwrap());
    // Accept Json
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let response = client
        .get(format!("{}/gateway/bot", DISCORD_API).as_str())
        .headers(headers)
        .send()
        .await;
    
    match response {
        Ok(mut response) => {
            println!("{:?}", response.status());
            println!("{:#?}", response.json::<BotGateway>().await.unwrap());
        },
        Err(e) => {
            panic!("{:?}", e);
        }
    };
}

async fn heart_beat_loop(client: Arc<Client>, heart_beat: u64) {
    loop {

        sleep(Duration::from_millis(heart_beat));
    }
}