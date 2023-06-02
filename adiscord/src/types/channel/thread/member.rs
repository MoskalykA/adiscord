use crate::types::guild::member::GuildMember;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ThreadMember {
    /// ID of the thread
    pub id: Option<String>,

    /// ID of the user
    pub user_id: Option<String>,

    /// Time the user last joined the thread
    pub join_timestamp: String,

    /// Any user-thread settings, currently only used for notifications
    pub flags: u8,

    /// Additional information about the user
    pub member: Option<GuildMember>,
}
