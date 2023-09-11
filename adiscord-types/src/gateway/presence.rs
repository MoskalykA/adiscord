use super::{activity::Activity, client, user::UserPartial};
use crate::Snowflake;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Update {
    /// User whose presence is being updated
    pub user: UserPartial,

    /// ID of the guild
    pub guild_id: Option<Snowflake>,

    /// Either "idle", "dnd", "online", or "offline"
    pub status: Option<String>,

    /// User's current activities
    pub activities: Option<Vec<Activity>>,

    /// User's platform-dependent status
    pub client_status: Option<client::Status>,
}
