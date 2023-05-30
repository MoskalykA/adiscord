use serde::Serialize;

#[derive(Serialize)]
pub struct GatewayIdentifyPresenceActivityButton {
    /// Text shown on the button (1-32 characters)
    pub label: String,

    /// URL opened when clicking the button (1-512 characters)
    pub url: String,
}
