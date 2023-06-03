use adiscord_intents::Intent;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

/// This type is used in the gateway system for callbacks.
pub type Callback = Arc<dyn Fn(Value) + Send + Sync>;

pub struct Gateway {
    pub intents: Vec<Intent>,
    pub callbacks: HashMap<String, Callback>,
    pub heartbeat_ack: bool,
}

impl Gateway {
    pub fn new() -> Self {
        Self {
            intents: Vec::new(),
            callbacks: HashMap::new(),
            heartbeat_ack: false,
        }
    }
}
