use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WelcomeScreen {
    description: Option<String>,
    welcome_channels: Vec<WelcomeScreenChannel>
}