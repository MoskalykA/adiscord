pub mod activity;
pub mod application;
pub mod bot;
pub mod channel;
pub mod client;
pub mod emoji;
pub mod guild;
pub mod identify;
pub mod integration;
pub mod invite;
pub mod member;
pub mod message;
pub mod opcode;
pub mod presence;
pub mod reaction;
pub mod ready;
pub mod role;
pub mod scheduled;
pub mod sticker;
pub mod thread;
pub mod typing;
pub mod user;
pub mod voice;
pub mod webhook;

use self::opcode::Opcode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct Gateway {
    /// Gateway opcode, which indicates the payload type
    pub op: Opcode,

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
