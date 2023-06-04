use super::{application, guild};
use crate::api::user::User;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Ready {
    /// API version
    pub v: u8,

    /// Information about the user including email
    pub user: User,

    /// Guilds the user is in
    pub guilds: Vec<guild::Unavailable>,

    /// Used for resuming connections
    pub session_id: String,

    /// Gateway URL for resuming connections
    pub resume_gateway_url: String,

    /// Shard information associated with this session, if sent when identifying
    pub shard: Option<(u16, u16)>,

    /// Contains id and flags
    pub application: application::Partial,
}
