use crate::api::guild::Member;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Start {
    /// ID of the channel
    pub channel_id: String,

    /// ID of the guild
    pub guild_id: Option<String>,

    /// ID of the user
    pub user_id: String,

    /// Unix time (in seconds) of when the user started typing
    pub timestamp: u64,

    /// Member who started typing if this happened in a guild
    pub member: Option<Member>,
}
