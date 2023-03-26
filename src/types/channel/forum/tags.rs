use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ForumTags {
    pub id: String,
    pub name: String,
    pub moderated: bool,
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>,
}
