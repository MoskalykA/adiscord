use crate::types::gateway::Gateway;
use crate::types::gateway::{identify::GatewayIdentify, opcode::GatewayOpcode::Identify};
use serde_json::{to_string, to_value};
use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};
use tungstenite::{stream::MaybeTlsStream, Message, WebSocket};

impl crate::Gateway {
    pub async fn identify(
        &self,
        socket: Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>,
        data: GatewayIdentify,
    ) {
        let identify = Gateway {
            op: Identify,
            d: Some(to_value(&data).unwrap()),
            s: None,
            t: None,
        };

        socket
            .lock()
            .unwrap()
            .write_message(Message::Text(to_string(&identify).unwrap()))
            .unwrap();
    }
}
