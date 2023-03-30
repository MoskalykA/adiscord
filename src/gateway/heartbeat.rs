use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};
use tungstenite::{stream::MaybeTlsStream, Message, WebSocket};

impl crate::Gateway {
    pub async fn send_heartbeat(&self, socket: &Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>) {
        socket
            .lock()
            .unwrap()
            .write_message(Message::Text(
                r#"{
                "op": 1,
                "d": null
            }"#
                .into(),
            ))
            .unwrap();
    }
}
