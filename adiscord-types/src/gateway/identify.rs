use super::activity::Activity;
use serde::Serialize;

#[derive(Serialize)]
pub struct IdentifyPresence {
    /// Unix time (in milliseconds) of when the client went idle, or null if the client is not idle
    pub since: Option<u64>,

    /// User's activities
    pub activities: Vec<Activity>,

    /// User's new status
    pub status: String,

    /// Whether or not the client is afk
    pub afk: bool,
}

#[derive(Serialize)]
pub struct IdentifyConnection {
    /// Your operating system
    pub os: String,

    /// Your library name
    pub browser: String,

    /// Your library name
    pub device: String,
}

#[derive(Serialize)]
pub struct Identify {
    /// Authentication token
    pub token: String,

    /// Connection properties
    pub properties: IdentifyConnection,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether this connection supports compression of packets
    pub compress: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Value between 50 and 250, total number of members where the gateway will stop sending offline members in the guild member list
    pub large_threshold: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Used for Guild Sharding
    pub shard: Option<(i32, i32)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Presence structure for initial presence information
    pub presence: Option<IdentifyPresence>,

    /// Gateway Intents you wish to receive
    pub intents: u32,
}
