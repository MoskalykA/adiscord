use super::r#type::ChannelType;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Mention {
    pub id: String,
    pub guild_id: String,
    pub r#type: ChannelType,
    pub name: String,
}
