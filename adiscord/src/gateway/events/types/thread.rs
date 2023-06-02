use crate::types::{
    channel::{
        r#type::ChannelType,
        thread::member::{ThreadMember},
        Channel,
    }
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ThreadDelete {
    /// the id of this channel
    pub id: String,

    /// the type of channel
    pub r#type: ChannelType,

    // the id of the guild (may be missing for some channel objects received over gateway guild dispatches)
    pub guild_id: Option<String>,

    /// for guild channels: id of the parent category for a channel (each parent category can contain up to 50 channels), for threads: id of the text channel this thread was created
    pub parent_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ThreadListSync {
    /// ID of the guild
    pub guild_id: String,

    /// Parent channel IDs whose threads are being synced. If omitted, then threads were synced for the entire guild. This array may contain channel_ids that have no active threads as well, so you know to clear that data.
    pub channel_ids: Option<Vec<String>>,

    /// All active threads in the given channels that the current user can access
    pub threads: Vec<Channel>,

    /// All thread member objects from the synced threads for the current user, indicating which threads the current user has been added to
    pub members: Vec<ThreadMember>,
}

#[derive(Deserialize, Debug)]
pub struct ThreadMemberUpdate {
    /// ID of the thread
    pub id: Option<String>,

    /// ID of the guild
    pub guild_id: String,

    /// ID of the user
    pub user_id: Option<String>,

    /// Time the user last joined the thread
    pub join_timestamp: String,

    /// Any user-thread settings, currently only used for notifications
    pub flags: u8,

    /// Additional information about the user
    pub member: Option<ThreadMember>,
}

#[derive(Deserialize, Debug)]
pub struct ThreadMembersUpdate {
    /// ID of the thread
    pub id: Option<String>,

    /// ID of the guild
    pub guild_id: String,

    /// Approximate number of members in the thread, capped at 50
    pub member_count: u8,

    /// Users who were added to the thread
    pub added_members: Option<Vec<ThreadMember>>,

    /// ID of the users who were removed from the thread
    pub removed_member_ids: Option<Vec<String>>,
}
