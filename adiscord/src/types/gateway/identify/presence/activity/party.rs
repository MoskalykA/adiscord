use serde::Serialize;

#[derive(Serialize)]
pub struct GatewayIdentifyPresenceActivityParty {
    /// ID of the party
    pub id: Option<String>,

    /// Used to show the party's current and maximum size
    pub size: Option<(u32, u32)>,
}
