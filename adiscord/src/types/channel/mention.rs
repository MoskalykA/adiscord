use super::r#type::ChannelType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Mention {
    /// id of the channel
    pub id: String,

    /// id of the guild containing the channel
    pub guild_id: String,

    /// the type of channel
    pub r#type: ChannelType,

    /// the name of the channel
    pub name: String,
}
