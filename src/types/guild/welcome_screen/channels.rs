use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WelcomeScreenChannels {
    /// the channel's id
    pub channel_id: String,

    /// the description shown for the channel
    pub description: String,

    /// the emoji id, if the emoji is custom
    pub emoji_id: String,

    /// the emoji name if custom, the unicode character if standard, or null if no emoji is set
    pub emoji_name: String,
}
