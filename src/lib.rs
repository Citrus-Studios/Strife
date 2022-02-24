use reqwest::header::{CONTENT_TYPE, CONTENT_LENGTH};


pub const DISCORD_API: &'static str = "https://discord.com/api";
pub const USER_AGENT_VAL: &'static str = "DiscordBot (https://github.com/Citrus-Studios, 0.0.1)";

// Early testing of the discord API
#[test]
fn api_test() {
    use reqwest::header::{HeaderMap, USER_AGENT, HeaderValue, AUTHORIZATION};

    let bot_token = "OTQ2NDc5Mjg3NzM1ODQwNzk4.YhfThg.2PHNRoLrczYKHYJzdaTK3g82WNs";

    let mut headers = HeaderMap::with_capacity(3);
    // Set the user agent header
    headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VAL));
    // Set the authorization header
    headers.insert(AUTHORIZATION, HeaderValue::from_str(bot_token).unwrap());
    // Set the content type header
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    // Content length
    headers.insert(CONTENT_LENGTH, HeaderValue::from_str("0").unwrap());
    
}