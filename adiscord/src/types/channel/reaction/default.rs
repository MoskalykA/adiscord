use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DefaultReaction {
    /// the id of a guild's custom emoji
    pub emoji_id: Option<String>,

    /// the unicode character of the emoji
    pub emoji_name: Option<String>,
}