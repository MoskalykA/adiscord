use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum PrivacyLevel {
    /// The Stage instance is visible publicly. (deprecated)
    PUBLIC = 1,

    /// The Stage instance is visible to only guild members.
    GUILD_ONLY,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StageInstance {
    /// The id of this Stage instance
    id: String,

    /// The guild id of the associated Stage channel
    guild_id: String,

    /// The id of the associated Stage channel
    channel_id: String,

    /// The topic of the Stage instance (1-120 characters)
    topic: String,

    /// The privacy level of the Stage instance
    privacy_level: PrivacyLevel,

    /// Whether or not Stage Discovery is disabled (deprecated)
    discoverable_disabled: bool,

    /// The id of the scheduled event for this Stage instance
    guild_scheduled_event_id: Option<String>,
}
