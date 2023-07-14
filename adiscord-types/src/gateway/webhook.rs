use serde::Deserialize;
use crate::Snowflake;

#[derive(Deserialize, Debug)]
pub struct Update {
    /// ID of the guild
    pub guild_id: Snowflake,

    /// ID of the channel
    pub channel_id: Snowflake,
}
