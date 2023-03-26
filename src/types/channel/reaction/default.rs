use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DefaultReaction {
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>,
}
