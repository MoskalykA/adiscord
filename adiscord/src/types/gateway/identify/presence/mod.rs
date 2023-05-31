pub mod activity;

use self::activity::GatewayIdentifyPresenceActivity;
use serde::Serialize;

#[derive(Serialize)]
pub struct GatewayIdentifyPresence {
    /// Unix time (in milliseconds) of when the client went idle, or null if the client is not idle
    pub since: Option<u64>,

    /// User's activities
    pub activities: Vec<GatewayIdentifyPresenceActivity>,

    /// User's new status
    pub status: String,

    /// Whether or not the client is afk
    pub afk: bool,
}
