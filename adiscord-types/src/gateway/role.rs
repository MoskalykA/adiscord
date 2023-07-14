use crate::{api::role::Role, Snowflake};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Create {
    /// ID of the guild
    pub guild_id: Snowflake,

    /// Role that was created
    pub role: Role,
}

#[derive(Deserialize, Debug)]
pub struct Update {
    /// ID of the guild
    pub guild_id: Snowflake,

    /// Role that was updated
    pub role: Role,
}

#[derive(Deserialize, Debug)]
pub struct Delete {
    /// ID of the guild
    pub guild_id: Snowflake,

    /// ID of the role
    pub role_id: Snowflake,
}
