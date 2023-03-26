use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WelcomeScreenChannels {
    pub channel_id: String,
    pub description: String,
    pub emoji_id: String,
    pub emoji_name: String,
}
