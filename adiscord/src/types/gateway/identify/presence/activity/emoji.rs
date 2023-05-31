use serde::Serialize;

#[derive(Serialize)]
pub struct GatewayIdentifyPresenceActivityEmoji {
    /// Name of the emoji
    pub name: String,

    /// ID of the emoji
    pub id: Option<String>,

    /// Whether the emoji is animated
    pub animated: Option<bool>,
}
