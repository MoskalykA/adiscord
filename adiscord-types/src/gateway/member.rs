use super::presence;
use crate::{
    api::{guild::Member, user::User},
    Snowflake,
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Add {
    /// ID of the guild
    pub guild_id: Snowflake,
}

#[derive(Deserialize, Debug)]
pub struct Remove {
    /// ID of the guild
    pub guild_id: Snowflake,

    /// User who was removed
    pub user: User,
}

#[derive(Deserialize, Debug)]
pub struct Update {
    /// ID of the guild
    pub guild_id: Snowflake,

    /// User role ids
    pub roles: Vec<String>,

    /// User
    pub user: User,

    /// Nickname of the user in the guild
    pub nick: Option<String>,

    /// Member's guild avatar hash
    pub avatar: Option<String>,

    /// When the user joined the guild
    pub joined_at: Option<String>,

    /// When the user starting boosting the guild
    pub premium_since: Option<String>,

    /// Whether the user is deafened in voice channels
    pub deaf: Option<bool>,

    /// Whether the user is muted in voice channels
    pub mute: Option<bool>,

    /// Whether the user has not yet passed the guild's Membership Screening requirements
    pub pending: Option<bool>,

    /// When the user's timeout will expire and the user will be able to communicate in the guild again, null or a time in the past if the user is not timed out
    pub communication_disabled_until: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Chunk {
    /// ID of the guild
    pub guild_id: Snowflake,

    /// Set of guild members
    pub members: Vec<Member>,

    /// Chunk index in the expected chunks for this response (0 <= chunk_index < chunk_count)
    pub chunk_index: u16,

    /// Total number of expected chunks for this response
    pub chunk_count: u16,

    /// When passing an invalid ID to REQUEST_GUILD_MEMBERS, it will be returned here
    pub not_found: Option<Value>,

    /// When passing true to REQUEST_GUILD_MEMBERS, presences of the returned members will be here
    pub presences: Option<Vec<presence::Update>>,

    /// Nonce used in the Guild Members Request
    pub nonce: Option<String>,
}
