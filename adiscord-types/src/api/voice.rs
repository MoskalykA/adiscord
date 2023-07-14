use crate::Snowflake;
use super::guild;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct VoiceRegion {
    /// unique ID for the region
    pub id: Snowflake,

    /// name of the region
    pub name: String,

    /// true for a single server that is closest to the current user's client
    pub optimal: bool,

    /// whether this is a deprecated voice region (avoid switching to these)
    pub deprecated: bool,

    /// whether this is a custom voice region (used for events/etc)
    pub custom: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VoiceState {
    /// the guild id this voice state is for
    pub guild_id: Option<Snowflake>,

    /// the channel id this user is connected to
    pub channel_id: Option<Snowflake>,

    /// the user id this voice state is for
    pub user_id: Snowflake,

    /// the guild member this voice state is for
    pub member: Option<guild::Member>,

    /// the session id for this voice state
    pub session_id: Snowflake,

    /// whether this user is deafened by the server
    pub deaf: bool,

    /// whether this user is muted by the server
    pub mute: bool,

    /// whether this user is locally deafened
    pub self_deaf: bool,

    /// whether this user is locally muted
    pub self_mute: bool,

    // whether this user is streaming using "Go Live"
    pub self_stream: Option<bool>,

    /// whether this user's camera is enabled
    pub self_video: bool,

    /// whether this user's permission to speak is denied
    pub suppress: bool,

    /// the time at which the user requested to speak
    pub request_to_speak_timestamp: Option<String>,
}
