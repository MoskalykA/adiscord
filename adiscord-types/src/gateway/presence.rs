use super::{activity::Activity, client};
use crate::api::user::User;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Update {
    /// User whose presence is being updated
    pub user: User,

    /// ID of the guild
    pub guild_id: String,

    /// Either "idle", "dnd", "online", or "offline"
    pub status: String,

    /// User's current activities
    pub activities: Vec<Activity>,

    /// User's platform-dependent status
    pub client_status: client::Status,
}
