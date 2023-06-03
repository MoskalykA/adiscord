use super::guild;
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
    pub member: Option<guild::Member>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ThreadMetadata {
    /// whether the thread is archived
    pub archived: bool,

    /// the thread will stop showing in the channel list after auto_archive_duration minutes of inactivity, can be set to: 60, 1440, 4320, 10080
    pub auto_archive_duration: u16,

    /// timestamp when the thread's archive status was last changed, used for calculating recent activity
    pub archive_timestamp: String,

    /// whether the thread is locked; when a thread is locked, only users with MANAGE_THREADS can unarchive it
    pub locked: bool,

    /// whether non-moderators can add other non-moderators to a thread; only available on private threads
    pub invitable: Option<bool>,

    /// timestamp when the thread was created; only populated for threads created after 2022-01-09
    pub create_timestamp: Option<String>,
}
