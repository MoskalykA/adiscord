use crate::api::emoji::Emoji;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DefaultReaction {
    /// the id of a guild's custom emoji
    pub emoji_id: Option<String>,

    /// the unicode character of the emoji
    pub emoji_name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Reaction {
    /// times this emoji has been used to react
    pub count: u16,

    /// whether the current user reacted using this emoji
    pub me: bool,

    /// emoji information
    pub emoji: Emoji,
}
