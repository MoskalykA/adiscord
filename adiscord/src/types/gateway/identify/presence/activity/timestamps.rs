use serde::Serialize;

#[derive(Serialize)]
pub struct GatewayIdentifyPresenceActivityTimestamps {
    /// Unix time (in milliseconds) of when the activity started
    pub start: Option<u64>,

    /// Unix time (in milliseconds) of when the activity ends
    pub end: Option<u64>,
}
