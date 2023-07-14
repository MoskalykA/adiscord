use crate::{api::emoji::Emoji, Snowflake};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Update {
    /// ID of the guild
    pub guild_id: Snowflake,

    /// Array of emojis
    pub emojis: Vec<Emoji>,
}
