use super::{emoji::Emoji, feature::Feature, role::Role, sticker::Sticker};
use serde_derive::Deserialize;
use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum VerificationLevel {
    NONE = 0,
    LOW = 1,
    MEDIUM = 2,
    HIGH = 3,
    VERY_HIGH = 4,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum DefaultMessageNotifications {
    ALL_MESSAGES = 0,
    ONLY_MENTIONS = 1,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ExplicitContentFilter {
    DISABLED = 0,
    MEMBERS_WITHOUT_ROLES = 1,
    ALL_MEMBERS = 2,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum MFALevel {
    None = 0,
    Elevated = 1,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SystemChannelFlags {
    None = 0,
    SuppressJoinNotifications = 1 << 0,
    SuppressPremiumSubscriptions = 1 << 1,
    SuppressGuildReminderNotifications = 1 << 2,
    SuppressJoinNotificationReplies = 1 << 3,
    SuppressRoleSubscriptionPurchaseNotifications = 1 << 4,
    SuppressRoleSubscriptionPurchaseNotificationReplies = 1 << 5,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum PremiumTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}

#[derive(Deserialize, Debug)]
pub struct WelcomeScreenChannels {
    pub channel_id: String,
    pub description: String,
    pub emoji_id: String,
    pub emoji_name: String,
}

#[derive(Deserialize, Debug)]
pub struct WelcomeScreen {
    pub description: String,
    pub welcome_channels: Vec<WelcomeScreenChannels>,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum NSFWLevel {
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
}

#[derive(Deserialize, Debug)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<bool>,
    pub owner_id: String,
    pub permissions: Option<String>,
    pub region: Option<String>,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: u16,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<String>,
    pub verification_level: VerificationLevel,
    pub default_message_notifications: DefaultMessageNotifications,
    pub explicit_content_filter: ExplicitContentFilter,
    pub roles: Vec<Role>,
    pub emojis: Vec<Emoji>,
    pub features: Vec<Feature>,
    pub mfa_level: MFALevel,
    pub application_id: Option<String>,
    pub system_channel_id: Option<String>,
    pub system_channel_flags: SystemChannelFlags,
    pub rules_channel_id: Option<String>,
    pub max_presences: Option<i128>,
    pub max_members: Option<i128>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: PremiumTier,
    pub premium_subscription_count: Option<i128>,
    pub preferred_locale: String,
    pub public_updates_channel_id: Option<String>,
    pub max_video_channel_users: Option<i128>,
    pub approximate_member_count: Option<i128>,
    pub approximate_presence_count: Option<i128>,
    pub welcome_screen: Option<WelcomeScreen>,
    pub nsfw_level: NSFWLevel,
    pub stickers: Vec<Sticker>,
    pub premium_progress_bar_enabled: bool,
}
