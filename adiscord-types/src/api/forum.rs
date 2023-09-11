use crate::Snowflake;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Tags {
    /// the id of the tag
    pub id: Snowflake,

    /// the name of the tag (0-20 characters)
    pub name: String,

    /// whether this tag can only be added to or removed from threads by a member with the MANAGE_THREADS permission
    pub moderated: bool,

    /// the id of a guild's custom emoji *
    pub emoji_id: Option<Snowflake>,

    /// the unicode character of the emoji *
    pub emoji_name: Option<String>,
}
