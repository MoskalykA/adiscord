pub mod flags;
pub mod forum;
pub mod forum_layout;
pub mod overwrite;
pub mod reaction;
pub mod sort_order;
pub mod thread;
pub mod r#type;
pub mod video;

use self::{
    flags::Flags,
    forum::tags::ForumTags,
    forum_layout::ForumLayout,
    overwrite::Overwrite,
    r#type::ChannelType,
    reaction::default::DefaultReaction,
    sort_order::SortOrder,
    thread::{member::ThreadMember, metadata::ThreadMetadata},
    video::VideoQuality,
};
use super::user::User;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Channel {
    pub id: String,
    pub r#type: ChannelType,
    pub guild_id: Option<String>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<u8>,
    pub rate_limit_per_user: Option<u16>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub managed: Option<bool>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<VideoQuality>,
    pub message_count: Option<i32>,
    pub member_count: Option<u8>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<u16>,
    pub permissions: Option<String>,
    pub flags: Option<Flags>,
    pub total_message_sent: Option<i32>,
    pub available_tags: Option<Vec<ForumTags>>,
    pub applied_tags: Option<Vec<String>>,
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub default_sort_order: Option<SortOrder>,
    pub default_forum_layout: Option<ForumLayout>,
}
