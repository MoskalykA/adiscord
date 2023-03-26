pub mod channels;

use self::channels::WelcomeScreenChannels;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WelcomeScreen {
    pub description: String,
    pub welcome_channels: Vec<WelcomeScreenChannels>,
}
