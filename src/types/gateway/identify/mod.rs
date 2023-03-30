pub mod connection;
pub mod presence;

use self::{connection::GatewayIdentifyConnection, presence::GatewayIdentifyPresence};
use serde::Serialize;

#[derive(Serialize)]
pub struct GatewayIdentify {
    /// Authentication token
    pub token: String,

    /// Connection properties
    pub properties: GatewayIdentifyConnection,

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
    pub presence: Option<GatewayIdentifyPresence>,

    /// Gateway Intents you wish to receive
    pub intents: u16,
}
