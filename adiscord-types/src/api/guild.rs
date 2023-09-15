use super::{emoji::Emoji, feature::Feature, role::Role, sticker::Sticker, user::User};
use crate::Snowflake;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum MemberFlags {
    /// Member has left and rejoined the guild
    DID_REJOIN = 1 << 0,

    /// Member has completed onboarding
    COMPLETED_ONBOARDING = 1 << 1,

    /// Member is exempt from guild verification requirements
    BYPASSES_VERIFICATION = 1 << 2,

    /// Member has started onboarding
    STARTED_ONBOARDING = 1 << 3,

    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Member {
    /// the user this guild member represents
    pub user: Option<User>,

    /// this user's guild nickname
    pub nick: Option<String>,

    /// the member's guild avatar hash
    pub avatar: Option<String>,

    /// array of role object ids
    pub roles: Vec<String>,

    /// when the user joined the guild
    pub joined_at: String,

    /// when the user started boosting the guild
    pub premium_since: Option<String>,

    /// whether the user is deafened in voice channels
    pub deaf: bool,

    /// whether the user is muted in voice channels
    pub mute: bool,

    /// guild member flags represented as a bit set, defaults to 0
    pub flags: MemberFlags,

    /// whether the user has not yet passed the guild's Membership Screening requirements
    pub pending: Option<bool>,

    /// total permissions of the member in the channel, including overwrites, returned when in the interaction object
    pub permissions: Option<String>,

    /// when the user's timeout will expire and the user will be able to communicate in the guild again, null or a time in the past if the user is not timed out
    pub communication_disabled_until: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Channels {
    /// the channel's id
    pub channel_id: Snowflake,

    /// the description shown for the channel
    pub description: String,

    /// the emoji id, if the emoji is custom
    pub emoji_id: Snowflake,

    /// the emoji name if custom, the unicode character if standard, or null if no emoji is set
    pub emoji_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WelcomeScreen {
    /// the server description shown in the welcome screen
    pub description: String,

    /// the channels shown in the welcome screen, up to 5
    pub welcome_channels: Vec<Channels>,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum DefaultMessageNotifications {
    /// members will receive notifications for all messages by default
    ALL_MESSAGES,

    /// members will receive notifications only for messages that @mention them by default
    ONLY_MENTIONS,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum ExplicitContentFilter {
    /// media content will not be scanned
    DISABLED,

    /// media content sent by members without roles will be scanned
    MEMBERS_WITHOUT_ROLES,

    /// media content sent by all members will be scanned
    ALL_MEMBERS,
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum MFALevel {
    /// guild has no MFA/2FA requirement for moderation actions
    None,

    /// guild has a 2FA requirement for moderation actions
    Elevated,
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum NSFWLevel {
    Default,
    Explicit,
    Safe,
    AgeRestricted,
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum PremiumTier {
    /// guild has not unlocked any Server Boost perks
    None,

    /// guild has unlocked Server Boost level 1 perks
    Tier1,

    /// guild has unlocked Server Boost level 2 perks
    Tier2,

    /// guild has unlocked Server Boost level 3 perks
    Tier3,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Preview {
    /// guild id
    pub id: Snowflake,

    /// guild name (2-100 characters, excluding trailing and leading whitespace)
    pub name: String,

    /// icon hash
    pub icon: Option<String>,

    /// splash hash
    pub splash: Option<String>,

    /// discovery splash hash; only present for guilds with the "DISCOVERABLE" feature
    pub discovery_splash: Option<String>,

    /// custom guild emojis
    pub emojis: Vec<Emoji>,

    /// enabled guild features
    pub features: Vec<Feature>,

    /// approximate number of members in this guild
    pub approximate_member_count: u32,

    /// approximate number of online members in this guild
    pub approximate_presence_count: u32,

    /// the description for the guild
    pub description: Option<String>,

    /// custom guild stickers
    pub stickers: Vec<Sticker>,
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum SystemChannelFlags {
    /// Suppress member join notifications
    SuppressJoinNotifications = 1 << 0,

    /// Suppress server boost notifications
    SuppressPremiumSubscriptions = 1 << 1,

    /// Suppress server setup tips
    SuppressGuildReminderNotifications = 1 << 2,

    /// Hide member join sticker reply buttons
    SuppressJoinNotificationReplies = 1 << 3,

    /// Suppress role subscription purchase and renewal notifications
    SuppressRoleSubscriptionPurchaseNotifications = 1 << 4,

    /// Hide role subscription sticker reply buttons
    SuppressRoleSubscriptionPurchaseNotificationReplies = 1 << 5,

    #[serde(other)]
    Unknown,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum VerificationLevel {
    /// unrestricted
    NONE,

    /// must have verified email on account
    LOW,

    /// must be registered on Discord for longer than 5 minutes
    MEDIUM,

    /// must be a member of the server for longer than 10 minutes
    HIGH,

    /// must have a verified phone number
    VERY_HIGH,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Permission {
    String(String),
    Number(u32),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Guild {
    /// guild id
    pub id: Snowflake,

    /// guild name (2-100 characters, excluding trailing and leading whitespace)
    pub name: String,

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
    pub owner_id: Snowflake,

    /// total permissions for the user in the guild (excludes overwrites and implicit permissions)
    pub permissions: Option<Permission>,

    /// voice region id for the guild (deprecated)
    pub region: Option<String>,

    /// id of afk channel
    pub afk_channel_id: Option<Snowflake>,

    /// afk timeout in seconds, can be set to: 60, 300, 900, 1800, 3600
    pub afk_timeout: u16,

    /// true if the server widget is enabled
    pub widget_enabled: Option<bool>,

    /// the channel id that the widget will generate an invite to, or null if set to no invite
    pub widget_channel_id: Option<Snowflake>,

    /// verification level required for the guild
    pub verification_level: VerificationLevel,

    /// default message notifications level
    pub default_message_notifications: DefaultMessageNotifications,

    /// explicit content filter level
    pub explicit_content_filter: ExplicitContentFilter,

    /// roles in the guild
    pub roles: Vec<Role>,

    /// custom guild emojis
    pub emojis: Vec<Emoji>,

    /// enabled guild features
    pub features: Vec<Feature>,

    /// required MFA level for the guild
    pub mfa_level: MFALevel,

    /// application id of the guild creator if it is bot-created
    pub application_id: Option<Snowflake>,

    /// the id of the channel where guild notices such as welcome messages and boost events are posted
    pub system_channel_id: Option<Snowflake>,

    /// system channel flags
    pub system_channel_flags: SystemChannelFlags,

    /// the id of the channel where Community guilds can display rules and/or guidelines
    pub rules_channel_id: Option<Snowflake>,

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
    pub premium_tier: PremiumTier,

    /// the number of boosts this guild currently has
    pub premium_subscription_count: Option<u16>,

    /// the preferred locale of a Community guild; used in server discovery and notices from Discord, and sent in interactions; defaults to "en-US"
    pub preferred_locale: String,

    /// the id of the channel where admins and moderators of Community guilds receive notices from Discord
    pub public_updates_channel_id: Option<Snowflake>,

    /// the maximum amount of users in a video channel
    pub max_video_channel_users: Option<u16>,

    /// the maximum amount of users in a stage video channel
    pub max_stage_video_channel_users: Option<u16>,

    /// approximate number of members in this guild, returned from the GET /guilds/<id> and /users/@me/guilds endpoints when with_counts is true
    pub approximate_member_count: Option<u32>,

    /// approximate number of non-offline members in this guild, returned from the GET /guilds/<id> and /users/@me/guilds  endpoints when with_counts is true
    pub approximate_presence_count: Option<u32>,

    /// the welcome screen of a Community guild, shown to new members, returned in an Invite's guild object
    pub welcome_screen: Option<WelcomeScreen>,

    /// guild NSFW level
    pub nsfw_level: NSFWLevel,

    /// custom guild stickers
    pub stickers: Vec<Sticker>,

    /// whether the guild has the boost progress bar enabled
    pub premium_progress_bar_enabled: bool,

    /// the id of the channel where admins and moderators of Community guilds receive safety alerts from Discord
    pub safety_alerts_channel_id: Option<Snowflake>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PartialGuild {
    /// guild id
    pub id: Snowflake,

    /// guild name (2-100 characters, excluding trailing and leading whitespace)
    pub name: String,

    /// icon hash
    pub icon: Option<String>,

    /// true if the user is the owner of the guild
    pub owner: bool,

    /// total permissions for the user in the guild (excludes overwrites and implicit permissions)
    pub permissions: Permission,

    /// enabled guild features
    pub features: Vec<Feature>,
}
