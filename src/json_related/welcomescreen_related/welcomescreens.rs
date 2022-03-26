use serde::{Deserialize, Serialize};

use super::welcomescreen_channels::WelcomeScreenChannel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WelcomeScreen {
    description: Option<String>,
    welcome_channels: Vec<WelcomeScreenChannel>
}