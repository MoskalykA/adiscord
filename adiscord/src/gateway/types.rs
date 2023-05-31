use super::Callback;
use adiscord_intents::Intent;
use std::collections::HashMap;

pub struct Gateway {
    pub token: String,
    pub intents: Vec<Intent>,
    pub callbacks: HashMap<String, Callback>,
    pub heartbeat_ack: bool,
}

impl Gateway {
    pub fn new(token: String) -> Self {
        Self {
            token,
            intents: Vec::new(),
            callbacks: HashMap::new(),
            heartbeat_ack: false,
        }
    }
}
