use crate::api::{emoji::Emoji, guild::Member};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Add {
    /// ID of the user
    pub user_id: String,

    /// ID of the channel
    pub channel_id: String,

    /// ID of the message
    pub message_id: String,

    /// ID of the guild
    pub guild_id: Option<String>,

    /// Member who reacted if this happened in a guild
    pub member: Option<Member>,

    /// Emoji used to react - example
    pub emoji: Emoji,
}

#[derive(Deserialize, Debug)]
pub struct Remove {
    /// ID of the user
    pub user_id: String,

    /// ID of the channel
    pub channel_id: String,

    /// ID of the message
    pub message_id: String,

    /// ID of the guild
    pub guild_id: Option<String>,

    /// Emoji used to react - example
    pub emoji: Emoji,
}

#[derive(Deserialize, Debug)]
pub struct RemoveAll {
    /// ID of the channel
    pub channel_id: String,

    /// ID of the message
    pub message_id: String,

    /// ID of the guild
    pub guild_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RemoveEmoji {
    /// ID of the channel
    pub channel_id: String,

    /// ID of the guild
    pub guild_id: Option<String>,

    /// ID of the message
    pub message_id: String,

    /// Emoji that was removed
    pub emoji: Emoji,
}
