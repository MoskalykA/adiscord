pub mod default_message_notifications;
pub mod explicit_content_filter;
pub mod member;
pub mod mfa_level;
pub mod nsfw_level;
pub mod premium_tier;
pub mod preview;
pub mod system_channel_flags;
pub mod verification_level;
pub mod welcome_screen;

use self::{
    default_message_notifications::DefaultMessageNotifications,
    explicit_content_filter::ExplicitContentFilter, mfa_level::MFALevel, nsfw_level::NSFWLevel,
    premium_tier::PremiumTier, system_channel_flags::SystemChannelFlags,
    verification_level::VerificationLevel, welcome_screen::WelcomeScreen,
};
use super::{emoji::Emoji, feature::Feature, role::Role, sticker::Sticker};
use serde_derive::Deserialize;

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
