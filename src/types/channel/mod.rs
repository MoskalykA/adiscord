pub mod attachment;
pub mod embed;
pub mod flags;
pub mod forum;
pub mod forum_layout;
pub mod mention;
pub mod message;
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
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Channel {
    /// the id of this channel
    pub id: String,

    /// the type of channel
    pub r#type: ChannelType,

    // the id of the guild (may be missing for some channel objects received over gateway guild dispatches)
    pub guild_id: Option<String>,

    /// sorting position of the channel
    pub position: Option<i32>,

    /// explicit permission overwrites for members and roles
    pub permission_overwrites: Option<Vec<Overwrite>>,

    /// the name of the channel (1-100 characters)
    pub name: Option<String>,

    /// the channel topic (0-4096 characters for GUILD_FORUM channels, 0-1024 characters for all others)
    pub topic: Option<String>,

    /// whether the channel is nsfw
    pub nsfw: Option<bool>,

    /// the id of the last message sent in this channel (or thread for GUILD_FORUM channels) (may not point to an existing or valid message or thread)
    pub last_message_id: Option<String>,

    /// the bitrate (in bits) of the voice channel
    pub bitrate: Option<i32>,

    /// the user limit of the voice channel
    pub user_limit: Option<u8>,

    /// amount of seconds a user has to wait before sending another message (0-21600); bots, as well as users with the permission manage_messages or manage_channel, are unaffected
    pub rate_limit_per_user: Option<u16>,

    /// the recipients of the DM
    pub recipients: Option<Vec<User>>,

    /// icon hash of the group DM
    pub icon: Option<String>,

    /// id of the creator of the group DM or thread
    pub owner_id: Option<String>,

    /// application id of the group DM creator if it is bot-created
    pub application_id: Option<String>,

    /// for group DM channels: whether the channel is managed by an application via the gdm.join OAuth2 scope
    pub managed: Option<bool>,

    /// for guild channels: id of the parent category for a channel (each parent category can contain up to 50 channels), for threads: id of the text channel this thread was created
    pub parent_id: Option<String>,

    /// when the last pinned message was pinned. This may be null in events such as GUILD_CREATE when a message is not pinned.
    pub last_pin_timestamp: Option<String>,

    /// voice region id for the voice channel, automatic when set to null
    pub rtc_region: Option<String>,

    /// the camera video quality mode of the voice channel, 1 when not present
    pub video_quality_mode: Option<VideoQuality>,

    /// number of messages (not including the initial message or deleted messages) in a thread.
    pub message_count: Option<i32>,

    /// an approximate count of users in a thread, stops counting at 50
    pub member_count: Option<u8>,

    /// thread-specific fields not needed by other channels
    pub thread_metadata: Option<ThreadMetadata>,

    /// thread member object for the current user, if they have joined the thread, only included on certain API endpoints
    pub member: Option<ThreadMember>,

    /// default duration, copied onto newly created threads, in minutes, threads will stop showing in the channel list after the specified period of inactivity, can be set to: 60, 1440, 4320, 10080
    pub default_auto_archive_duration: Option<u16>,

    /// computed permissions for the invoking user in the channel, including overwrites, only included when part of the resolved data received on a slash command interaction
    pub permissions: Option<String>,

    /// channel flags combined as a bitfield
    pub flags: Option<Flags>,

    /// number of messages ever sent in a thread, it's similar to message_count on message creation, but will not decrement the number when a message is deleted
    pub total_message_sent: Option<i32>,

    /// the set of tags that can be used in a GUILD_FORUM channel
    pub available_tags: Option<Vec<ForumTags>>,

    /// the IDs of the set of tags that have been applied to a thread in a GUILD_FORUM channel
    pub applied_tags: Option<Vec<String>>,

    /// the emoji to show in the add reaction button on a thread in a GUILD_FORUM channel
    pub default_reaction_emoji: Option<DefaultReaction>,

    /// the initial rate_limit_per_user to set on newly created threads in a channel. this field is copied to the thread at creation time and does not live update.
    pub default_thread_rate_limit_per_user: Option<i32>,

    /// the default sort order type used to order posts in GUILD_FORUM channels. Defaults to null, which indicates a preferred sort order hasn't been set by a channel admin
    pub default_sort_order: Option<SortOrder>,

    /// the default forum layout view used to display posts in GUILD_FORUM channels. Defaults to 0, which indicates a layout view has not been set by a channel admin
    pub default_forum_layout: Option<ForumLayout>,
}
