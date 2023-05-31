pub mod bot;
pub mod identify;
pub mod opcode;

use self::opcode::GatewayOpcode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct Gateway {
    /// Gateway opcode, which indicates the payload type
    pub op: GatewayOpcode,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Event data
    pub d: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sequence number of event used for resuming sessions and heartbeating
    pub s: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Event name
    pub t: Option<String>,
}
