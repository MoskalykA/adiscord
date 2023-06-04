use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Update {
    /// ID of the guild
    pub guild_id: String,

    /// ID of the channel
    pub channel_id: String,
}
