#![feature(async_closure)]

use std::sync::Arc;

use client::Client;
use reqwest::header::ACCEPT;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use reqwest::Client as ReqwestClient;
use strife_types::bot_gateway::BotGateway;
use token::bot_token;
use tracing::info;

pub mod client;
pub mod token;

pub const DISCORD_API: &'static str = "https://discord.com/api/v9";
pub const USER_AGENT_VAL: &'static str = "Strife (https://github.com/Citrus-Studios, 0.0.1)";

pub type Snowflake = u64;
pub type Timestamp = String;

// Early testing of the discord API
#[allow(dead_code)]
async fn api_test() {
    info!("this shit even working?!!!");
    let client = Arc::new(ReqwestClient::new());
    let mut headers = HeaderMap::with_capacity(3);
    // Set the user agent header
    headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VAL));
    // Set the authorization header
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bot {}", bot_token)).unwrap(),
    );
    // Accept Json
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let response = client
        .get(&format!("{}/gateway/bot", DISCORD_API))
        .headers(headers.clone())
        .send()
        .await
        .unwrap();

    let bot_gateway = Arc::new(response.json::<BotGateway>().await.unwrap());

    let heartbeat = Client::new(bot_gateway.clone());
    heartbeat.run(bot_token.to_string()).await;
}

#[tokio::test]
async fn test_api_test_fn() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    api_test().await;
}
