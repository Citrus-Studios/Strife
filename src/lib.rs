use reqwest::header::{HeaderMap, USER_AGENT, HeaderValue, AUTHORIZATION};



pub const DISCORD_API: &'static str = "https://discord.com/api";
pub const USER_AGENT_VAL: &'static str = "DiscordBot (Null, 0.0.1)";

#[test]
fn api_test() {
    let bot_token = "";

    let mut headers = HeaderMap::with_capacity(3);
    // Set the user agent header
    headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VAL));
    // Set the authorization header
    headers.insert(AUTHORIZATION, HEADE)
}