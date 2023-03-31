use rand::{thread_rng, Rng};
use std::time::Duration;
use tokio::time::sleep;

impl crate::Gateway {
    pub async fn wait_heartbeat(&self, heartbeat_interval: f64) {
        let mut rng = thread_rng();
        let jitter: f64 = rng.gen();
        sleep(Duration::from_millis((heartbeat_interval * jitter) as u64)).await;
    }
}
