use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ForumTags {
    /// the id of the tag
    pub id: String,

    /// the name of the tag (0-20 characters)
    pub name: String,

    /// whether this tag can only be added to or removed from threads by a member with the MANAGE_THREADS permission
    pub moderated: bool,

    /// the id of a guild's custom emoji *
    pub emoji_id: Option<String>,

    /// the unicode character of the emoji *
    pub emoji_name: Option<String>,
}
