pub mod opcode;

use self::opcode::GatewayOpcode;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Gateway {
    /// Gateway opcode, which indicates the payload type
    pub op: GatewayOpcode,

    /// Event data
    pub d: Option<Value>,

    /// Sequence number of event used for resuming sessions and heartbeating
    pub s: Option<u16>,

    /// Event name
    pub t: Option<String>,
}
