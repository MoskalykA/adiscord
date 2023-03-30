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
