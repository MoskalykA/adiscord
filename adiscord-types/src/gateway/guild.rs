use super::presence;
use crate::api::{
    channel::Channel,
    emoji::Emoji,
    feature::Feature,
    guild::{
        self, DefaultMessageNotifications, ExplicitContentFilter, MFALevel, NSFWLevel, PremiumTier,
        SystemChannelFlags, VerificationLevel, WelcomeScreen,
    },
    role::Role,
    stage::{PrivacyLevel, StageInstance},
    sticker::Sticker,
    user::User,
    voice::VoiceState,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Unavailable {
    /// guild id
    pub id: String,

    /// unavailable or not
    pub unavailable: Option<bool>,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
pub enum EventStatus {
    SCHEDULED = 1,
    ACTIVE,
    COMPLETED,
    CANCELED,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
pub enum EventEntityTypes {
    STAGE_INSTANCE = 1,
    VOICE,
    EXTERNAL,
}

#[derive(Deserialize, Debug)]
pub struct EventEntityMetadata {
    /// location of the event (1-100 characters)
    pub location: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Scheduled {
    /// the id of the scheduled event
    pub id: String,

    /// the guild id which the scheduled event belongs to
    pub guild_id: String,

    /// the channel id in which the scheduled event will be hosted, or null if scheduled entity type is EXTERNAL
    pub channel_id: Option<String>,

    /// the id of the user that created the scheduled event *
    pub creator_id: Option<String>,

    /// the name of the scheduled event (1-100 characters)
    pub name: String,

    /// the description of the scheduled event (1-1000 characters)
    pub description: Option<String>,

    /// the time the scheduled event will start
    pub scheduled_start_time: String,

    /// the time the scheduled event will end, required if entity_type is EXTERNAL
    pub scheduled_end_time: Option<String>,

    /// the privacy level of the scheduled event
    pub privacy_level: PrivacyLevel,

    /// the status of the scheduled event
    pub status: EventStatus,

    /// the type of the scheduled event
    pub entity_type: EventEntityTypes,

    /// the id of an entity associated with a guild scheduled event
    pub entity_id: Option<String>,

    /// additional metadata for the guild scheduled event
    pub entity_metadata: Option<EventEntityMetadata>,

    /// the user that created the scheduled event
    pub creator: Option<User>,

    /// the number of users subscribed to the scheduled event
    pub user_count: Option<u16>,

    /// the cover image hash of the scheduled event
    pub image: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Create {
    /// guild id
    pub id: String,

    /// GUILD AVAILABLE:
    ///     true if this guild is unavailable due to an outage
    ///
    /// GUILD UNVAILABLE:
    ///     unavailable or not
    pub unavailable: Option<bool>,

    // GUILD SECTION
    /// guild name (2-100 characters, excluding trailing and leading whitespace)
    pub name: Option<String>,

    /// icon hash
    pub icon: Option<String>,

    /// icon hash, returned when in the template object
    pub icon_hash: Option<String>,

    /// splash hash
    pub splash: Option<String>,

    /// discovery splash hash; only present for guilds with the "DISCOVERABLE" feature
    pub discovery_splash: Option<String>,

    /// true if the user is the owner of the guild
    pub owner: Option<bool>,

    /// id of owner
    pub owner_id: Option<String>,

    /// total permissions for the user in the guild (excludes overwrites)
    pub permissions: Option<String>,

    /// voice region id for the guild (deprecated)
    pub region: Option<String>,

    /// id of afk channel
    pub afk_channel_id: Option<String>,

    /// afk timeout in seconds, can be set to: 60, 300, 900, 1800, 3600
    pub afk_timeout: Option<u16>,

    /// true if the server widget is enabled
    pub widget_enabled: Option<bool>,

    /// the channel id that the widget will generate an invite to, or null if set to no invite
    pub widget_channel_id: Option<String>,

    /// verification level required for the guild
    pub verification_level: Option<VerificationLevel>,

    /// default message notifications level
    pub default_message_notifications: Option<DefaultMessageNotifications>,

    /// explicit content filter level
    pub explicit_content_filter: Option<ExplicitContentFilter>,

    /// roles in the guild
    pub roles: Option<Vec<Role>>,

    /// custom guild emojis
    pub emojis: Vec<Emoji>,

    /// enabled guild features
    pub features: Option<Vec<Feature>>,

    /// required MFA level for the guild
    pub mfa_level: Option<MFALevel>,

    /// application id of the guild creator if it is bot-created
    pub application_id: Option<String>,

    /// the id of the channel where guild notices such as welcome messages and boost events are posted
    pub system_channel_id: Option<String>,

    /// system channel flags
    pub system_channel_flags: Option<SystemChannelFlags>,

    /// the id of the channel where Community guilds can display rules and/or guidelines
    pub rules_channel_id: Option<String>,

    /// the maximum number of presences for the guild (null is always returned, apart from the largest of guilds)
    pub max_presences: Option<u32>,

    /// the maximum number of members for the guild
    pub max_members: Option<u32>,

    /// the vanity url code for the guild
    pub vanity_url_code: Option<String>,

    /// the description of a guild
    pub description: Option<String>,

    /// banner hash
    pub banner: Option<String>,

    /// premium tier (Server Boost level)
    pub premium_tier: Option<PremiumTier>,

    /// the number of boosts this guild currently has
    pub premium_subscription_count: Option<u16>,

    /// the preferred locale of a Community guild; used in server discovery and notices from Discord, and sent in interactions; defaults to "en-US"
    pub preferred_locale: Option<String>,

    /// the id of the channel where admins and moderators of Community guilds receive notices from Discord
    pub public_updates_channel_id: Option<String>,

    /// the maximum amount of users in a video channel
    pub max_video_channel_users: Option<u16>,

    /// the maximum amount of users in a stage video channel
    pub max_stage_video_channel_users: Option<u16>,

    /// approximate number of members in this guild, returned from the GET /guilds/<id> endpoint when with_counts is true
    pub approximate_member_count: Option<u32>,

    /// approximate number of non-offline members in this guild, returned from the GET /guilds/<id> endpoint when with_counts is true
    pub approximate_presence_count: Option<u32>,

    /// the welcome screen of a Community guild, shown to new members, returned in an Invite's guild object
    pub welcome_screen: Option<WelcomeScreen>,

    /// guild NSFW level
    pub nsfw_level: Option<NSFWLevel>,

    /// custom guild stickers
    pub stickers: Option<Vec<Sticker>>,

    /// whether the guild has the boost progress bar enabled
    pub premium_progress_bar_enabled: Option<bool>,

    /// the id of the channel where admins and moderators of Community guilds receive safety alerts from Discord
    pub safety_alerts_channel_id: Option<String>,

    /// When this guild was joined at
    pub joined_at: String,

    /// true if this is considered a large guild
    pub large: bool,

    /// Total number of members in this guild
    pub member_count: u32,

    /// States of members currently in voice channels; lacks the guild_id key
    pub voice_states: Vec<VoiceState>,

    /// Users in the guild
    pub members: Vec<guild::Member>,

    /// Channels in the guild
    pub channels: Vec<Channel>,

    /// All active threads in the guild that current user has permission to view
    pub threads: Vec<Channel>,

    /// Presences of the members in the guild, will only include non-offline members if the size is greater than large threshold
    pub presences: Vec<presence::Update>,

    /// Stage instances in the guild
    pub stage_instances: Vec<StageInstance>,

    /// Scheduled events in the guild
    pub guild_scheduled_events: Vec<Scheduled>,
}

#[derive(Deserialize, Debug)]
pub struct BanAdd {
    /// ID of the guild
    pub guild_id: String,

    /// User who was banned
    pub user: User,
}

#[derive(Deserialize, Debug)]
pub struct BanRemove {
    /// ID of the guild
    pub guild_id: String,

    /// User who was banned
    pub user: User,
}
