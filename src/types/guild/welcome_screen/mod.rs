pub mod channels;

use self::channels::WelcomeScreenChannels;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WelcomeScreen {
    /// the server description shown in the welcome screen
    pub description: String,

    /// the channels shown in the welcome screen, up to 5
    pub welcome_channels: Vec<WelcomeScreenChannels>,
}
