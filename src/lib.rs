use reqwest::Client;
use reqwest::header::{CONTENT_TYPE, CONTENT_LENGTH, ACCEPT};
use reqwest::header::{HeaderMap, USER_AGENT, HeaderValue, AUTHORIZATION};

pub const DISCORD_API: &'static str = "https://discord.com/api/v9/";
pub const USER_AGENT_VAL: &'static str = "Strife (https://github.com/Citrus-Studios, 0.0.1)";

#[tokio::test]
async fn test_api_test_fn() {
    api_test().await;
}

// Early testing of the discord API
async fn api_test() {

    let bot_token = "Bot OTQ2NDc5Mjg3NzM1ODQwNzk4.YhfThg.2PHNRoLrczYKHYJzdaTK3g82WNs";

    let client = Client::new();
    let mut headers = HeaderMap::with_capacity(5);
    // Set the user agent header
    headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VAL));
    // Set the authorization header
    headers.insert(AUTHORIZATION, HeaderValue::from_str(bot_token).unwrap());
    // Set the content type header
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    // Accept Json
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    // Length
    headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));

    
    let response = client
        .post(DISCORD_API)
        .headers(headers)
        .send()
        .await;
    
    match response {
        Ok(mut response) => {
            // println!("{:?}", response);
            println!("{:?}", response.status());
            // println!("{:?}", response.headers());
            println!("{:?}", response.text().await);
        },
        Err(e) => {
            panic!("{:?}", e);
        }
    };
}