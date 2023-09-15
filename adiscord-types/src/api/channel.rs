use super::{
    forum,
    reaction::DefaultReaction,
    thread::{ThreadMember, ThreadMetadata},
    user::User,
};
use crate::Snowflake;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum AttachmentFlags {
    /// this attachment has been edited using the remix feature on mobile
    IS_REMIX = 1 << 2,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Attachment {
    /// attachment id
    pub id: Snowflake,

    /// name of file attached
    pub filename: String,

    /// description for the file (max 1024 characters)
    pub description: Option<String>,

    /// the attachment's media type
    pub content_type: Option<String>,

    /// size of file in bytes
    pub size: u32,

    /// source url of file
    pub url: String,

    /// a proxied url of file
    pub proxy_url: String,

    /// height of file (if image)
    pub height: Option<u16>,

    /// width of file (if image)
    pub width: Option<u16>,

    /// whether this attachment is ephemeral
    pub ephemeral: Option<bool>,

    /// the duration of the audio file (currently for voice messages)
    pub duration_secs: Option<f32>,

    /// base64 encoded bytearray representing a sampled waveform (currently for voice messages)
    pub waveform: Option<String>,

    /// attachment flags combined as a bitfield
    pub flags: Option<AttachmentFlags>,
}

#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum Flags {
    /// this thread is pinned to the top of its parent GUILD_FORUM or GUILD_MEDIA channel
    PINNED = 1 << 1,

    /// whether a tag is required to be specified when creating a thread in a GUILD_FORUM or a GUILD_MEDIA channel. Tags are specified in the applied_tags field.
    REQUIRE_TAG = 1 << 4,

    /// when set hides the embedded media download options. Available only for media channels
    HIDE_MEDIA_DOWNLOAD_OPTIONS = 1 << 15,

    #[serde(other)]
    Unknown,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum ForumLayout {
    /// No default has been set for forum channel
    NOT_SET,

    /// Display posts as a list
    LIST_VIEW,

    /// Display posts as a collection of tiles
    GALLERY_VIEW,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Mention {
    /// id of the channel
    pub id: Snowflake,

    /// id of the guild containing the channel
    pub guild_id: Snowflake,

    /// the type of channel
    pub r#type: ChannelType,

    /// the name of the channel
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Overwrite {
    /// role or user id
    pub id: Snowflake,

    /// either 0 (role) or 1 (member)
    pub r#type: u8,

    /// permission bit set
    pub allow: String,

    /// permission bit set
    pub deny: String,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum SortOrder {
    /// Sort forum posts by activity
    LATEST_ACTIVITY,

    /// Sort forum posts by creation time (from most recent to oldest)
    CREATION_DATE,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum ChannelType {
    /// atext channel within a server
    GUILD_TEXT,

    /// a direct message between users
    DM,

    /// a voice channel within a server
    GUILD_VOICE,

    /// a direct message between multiple users
    GROUP_DM,

    /// an organizational category that contains up to 50 channels
    GUILD_CATEGORY,

    /// a channel that users can follow and crosspost into their own server (formerly news channels)
    GUILD_ANNOUNCEMENT,

    /// a temporary sub-channel within a GUILD_ANNOUNCEMENT channel
    ANNOUNCEMENT_THREAD = 10,

    /// a temporary sub-channel within a GUILD_TEXT or GUILD_FORUM channel
    PUBLIC_THREAD,

    /// a temporary sub-channel within a GUILD_TEXT channel that is only viewable by those invited and those with the MANAGE_THREADS permission
    PRIVATE_THREAD,

    /// a voice channel for hosting events with an audience
    GUILD_STAGE_VOICE,

    /// the channel in a hub containing the listed servers
    GUILD_DIRECTORY,

    /// Channel that can only contain threads
    GUILD_FORUM,

    /// Channel that can only contain threads, similar to GUILD_FORUM channels
    GUILD_MEDIA,
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum VideoQuality {
    /// Discord chooses the quality for optimal performance
    AUTO = 1,

    /// 720p
    FULL,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Channel {
    /// the id of this channel
    pub id: Snowflake,

    /// the type of channel
    pub r#type: ChannelType,

    // the id of the guild (may be missing for some channel objects received over gateway guild dispatches)
    pub guild_id: Option<Snowflake>,

    /// sorting position of the channel
    pub position: Option<i32>,

    /// explicit permission overwrites for members and roles
    pub permission_overwrites: Option<Vec<Overwrite>>,

    /// the name of the channel (1-100 characters)
    pub name: Option<String>,

    /// the channel topic (0-4096 characters for GUILD_FORUM and GUILD_MEDIA channels, 0-1024 characters for all others)
    pub topic: Option<String>,

    /// whether the channel is nsfw
    pub nsfw: Option<bool>,

    /// the id of the last message sent in this channel (or thread for GUILD_FORUM or GUILD_MEDIA channels) (may not point to an existing or valid message or thread)
    pub last_message_id: Option<Snowflake>,

    /// the bitrate (in bits) of the voice channel
    pub bitrate: Option<i32>,

    /// the user limit of the voice channel
    pub user_limit: Option<u16>,

    /// amount of seconds a user has to wait before sending another message (0-21600); bots, as well as users with the permission manage_messages or manage_channel, are unaffected
    pub rate_limit_per_user: Option<u16>,

    /// the recipients of the DM
    pub recipients: Option<Vec<User>>,

    /// icon hash of the group DM
    pub icon: Option<String>,

    /// id of the creator of the group DM or thread
    pub owner_id: Option<Snowflake>,

    /// application id of the group DM creator if it is bot-created
    pub application_id: Option<Snowflake>,

    /// for group DM channels: whether the channel is managed by an application via the gdm.join OAuth2 scope
    pub managed: Option<bool>,

    /// for guild channels: id of the parent category for a channel (each parent category can contain up to 50 channels), for threads: id of the text channel this thread was created
    pub parent_id: Option<Snowflake>,

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

    /// computed permissions for the invoking user in the channel, including overwrites, only included when part of the resolved data received on a slash command interaction. This does not include implicit permissions, which may need to be checked separately
    pub permissions: Option<String>,

    /// channel flags combined as a bitfield
    pub flags: Option<Flags>,

    /// number of messages ever sent in a thread, it's similar to message_count on message creation, but will not decrement the number when a message is deleted
    pub total_message_sent: Option<i32>,

    /// the set of tags that can be used in a GUILD_FORUM or a GUILD_MEDIA channel
    pub available_tags: Option<Vec<forum::Tags>>,

    /// the IDs of the set of tags that have been applied to a thread in a GUILD_FORUM or a GUILD_MEDIA channel
    pub applied_tags: Option<Vec<Snowflake>>,

    /// the emoji to show in the add reaction button on a thread in a GUILD_FORUM or a GUILD_MEDIA channel
    pub default_reaction_emoji: Option<DefaultReaction>,

    /// the initial rate_limit_per_user to set on newly created threads in a channel. this field is copied to the thread at creation time and does not live update.
    pub default_thread_rate_limit_per_user: Option<i32>,

    /// the default sort order type used to order posts in GUILD_FORUM and GUILD_MEDIA channels. Defaults to null, which indicates a preferred sort order hasn't been set by a channel admin
    pub default_sort_order: Option<SortOrder>,

    /// the default forum layout view used to display posts in GUILD_FORUM channels. Defaults to 0, which indicates a layout view has not been set by a channel admin
    pub default_forum_layout: Option<ForumLayout>,
}
