use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SessionStartLimit {
    /// Total number of session starts the current user is allowed
    pub total: u32,

    /// Remaining number of session starts the current user is allowed
    pub remaining: u32,

    /// Number of milliseconds after which the limit resets
    pub reset_after: u32,

    /// Number of identify requests allowed per 5 seconds
    pub max_concurrency: u32,
}

#[derive(Debug, Deserialize)]
pub struct Bot {
    /// WSS URL that can be used for connecting to the Gateway
    pub url: String,

    /// Recommended number of shards to use when connecting
    pub shards: u8,

    /// Information on the current session start limit
    pub session_start_limit: SessionStartLimit,
}
