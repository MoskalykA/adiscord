use crate::api::emoji::Emoji;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Update {
    /// ID of the guild
    pub guild_id: String,

    /// Array of emojis
    pub emojis: Vec<Emoji>,
}
