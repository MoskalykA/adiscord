use crate::Snowflake;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PinsUpdate {
    /// ID of the guild
    pub guild_id: Option<Snowflake>,

    /// ID of the channel
    pub channel_id: Snowflake,

    /// Time at which the most recent pinned message was pinned
    pub last_pin_timestamp: Option<String>,
}
