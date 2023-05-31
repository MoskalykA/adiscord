pub mod assets;
pub mod button;
pub mod emoji;
pub mod party;
pub mod secrets;
pub mod timestamps;
pub mod r#type;

use self::{
    assets::GatewayIdentifyPresenceActivityAssets, button::GatewayIdentifyPresenceActivityButton,
    emoji::GatewayIdentifyPresenceActivityEmoji, party::GatewayIdentifyPresenceActivityParty,
    r#type::GatewayIdentifyPresenceActivityType, secrets::GatewayIdentifyPresenceActivitySecrets,
    timestamps::GatewayIdentifyPresenceActivityTimestamps,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct GatewayIdentifyPresenceActivity {
    /// Activity's name
    pub name: String,

    /// Activity type
    pub r#type: GatewayIdentifyPresenceActivityType,

    /// Stream URL, is validated when type is 1
    pub url: Option<String>,

    /// Unix timestamp (in milliseconds) of when the activity was added to the user's session
    pub created_at: u64,

    /// Unix timestamps for start and/or end of the game
    pub timestamps: Option<GatewayIdentifyPresenceActivityTimestamps>,

    /// Application ID for the game
    pub application_id: Option<String>,

    /// What the player is currently doing
    pub details: Option<String>,

    /// User's current party status
    pub state: Option<String>,

    /// Emoji used for a custom status
    pub emoji: Option<GatewayIdentifyPresenceActivityEmoji>,

    /// Information for the current party of the player
    pub party: Option<GatewayIdentifyPresenceActivityParty>,

    /// Images for the presence and their hover texts
    pub assets: Option<GatewayIdentifyPresenceActivityAssets>,

    /// Secrets for Rich Presence joining and spectating
    pub secrets: Option<GatewayIdentifyPresenceActivitySecrets>,

    /// Whether or not the activity is an instanced game session
    pub instance: Option<bool>,

    /// Activity flags ORd together, describes what the payload includes
    pub flags: Option<u8>,

    /// Custom buttons shown in the Rich Presence (max 2)
    pub buttons: Option<Vec<GatewayIdentifyPresenceActivityButton>>,
}
