use serde::Serialize;

#[derive(Serialize)]
pub struct GatewayIdentifyConnection {
    /// Your operating system
    pub os: String,

    /// Your library name
    pub browser: String,

    /// Your library name
    pub device: String,
}
