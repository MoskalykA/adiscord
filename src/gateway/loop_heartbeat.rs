use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::interval;
use tungstenite::{stream::MaybeTlsStream, WebSocket};

impl crate::Gateway {
    pub async fn loop_heartbeat(
        &self,
        socket: &Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>,
        heartbeat_interval: u64,
    ) {
        let socket = std::sync::Arc::clone(&socket);
        let gateway = std::sync::Arc::new(self.clone());
        let duration = Duration::from_millis(heartbeat_interval);

        tokio::spawn(async move {
            let mut interval = interval(duration);
            loop {
                interval.tick().await;

                gateway.send_heartbeat(&socket).await;
            }
        });
    }
}
