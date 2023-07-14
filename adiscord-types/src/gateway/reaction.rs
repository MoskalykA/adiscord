use crate::{api::{emoji::Emoji, guild::Member}, Snowflake};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Add {
    /// ID of the user
    pub user_id: Snowflake,

    /// ID of the channel
    pub channel_id: Snowflake,

    /// ID of the message
    pub message_id: Snowflake,

    /// ID of the guild
    pub guild_id: Option<Snowflake>,

    /// Member who reacted if this happened in a guild
    pub member: Option<Member>,

    /// Emoji used to react - example
    pub emoji: Emoji,
}

#[derive(Deserialize, Debug)]
pub struct Remove {
    /// ID of the user
    pub user_id: Snowflake,

    /// ID of the channel
    pub channel_id: Snowflake,

    /// ID of the message
    pub message_id: Snowflake,

    /// ID of the guild
    pub guild_id: Option<Snowflake>,

    /// Emoji used to react - example
    pub emoji: Emoji,
}

#[derive(Deserialize, Debug)]
pub struct RemoveAll {
    /// ID of the channel
    pub channel_id: Snowflake,

    /// ID of the message
    pub message_id: Snowflake,

    /// ID of the guild
    pub guild_id: Option<Snowflake>,
}

#[derive(Deserialize, Debug)]
pub struct RemoveEmoji {
    /// ID of the channel
    pub channel_id: Snowflake,

    /// ID of the guild
    pub guild_id: Option<Snowflake>,

    /// ID of the message
    pub message_id: Snowflake,

    /// Emoji that was removed
    pub emoji: Emoji,
}
