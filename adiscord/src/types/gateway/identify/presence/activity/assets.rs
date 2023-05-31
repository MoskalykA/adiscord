use serde::Serialize;

#[derive(Serialize)]
pub struct GatewayIdentifyPresenceActivityAssets {
    /// See Activity Asset Image
    pub large_image: Option<String>,

    /// Text displayed when hovering over the large image of the activity
    pub large_text: Option<String>,

    /// See Activity Asset Image
    pub small_image: Option<String>,

    /// Text displayed when hovering over the small image of the activity
    pub small_text: Option<String>,
}
