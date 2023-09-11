use crate::{api::emoji::Emoji, Snowflake};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
pub struct DefaultReaction {
    /// the id of a guild's custom emoji
    pub emoji_id: Option<Snowflake>,

    /// the unicode character of the emoji
    pub emoji_name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CountDetails {
    /// Count of super reactions
    burst: u16,

    /// Count of normal reactions
    normal: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Reaction {
    /// Total number of times this emoji has been used to react (including super reacts)
    pub count: u16,

    /// Reaction count details object
    pub count_details: CountDetails,

    /// Whether the current user reacted using this emoji
    pub me: bool,

    /// Whether the current user super-reacted using this emoji
    pub me_burst: bool,

    /// emoji information
    pub emoji: Emoji,

    /// HEX colors used for super reaction
    pub burst_colors: Value,
}
