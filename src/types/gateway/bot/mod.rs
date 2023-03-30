pub mod session_start_limit;

use self::session_start_limit::SessionStartLimit;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GatewayBot {
    /// WSS URL that can be used for connecting to the Gateway
    pub url: String,

    /// Recommended number of shards to use when connecting
    pub shards: u8,

    /// Information on the current session start limit
    pub session_start_limit: SessionStartLimit,
}
