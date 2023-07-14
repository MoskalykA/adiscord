use serde::Deserialize;
use crate::Snowflake;

#[derive(Deserialize, Debug)]
pub struct UserAdd {
    /// ID of the guild scheduled event
    pub guild_scheduled_event_id: Snowflake,

    /// ID of the user
    pub user_id: Snowflake,

    /// ID of the guild
    pub guild_id: Snowflake,
}

#[derive(Deserialize, Debug)]
pub struct UserRemove {
    /// ID of the guild scheduled event
    pub guild_scheduled_event_id: Snowflake,

    /// ID of the user
    pub user_id: Snowflake,

    /// ID of the guild
    pub guild_id: Snowflake,
}
