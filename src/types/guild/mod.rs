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
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Guild {
    /// guild id
    pub id: String,

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
    pub owner_id: String,

    /// total permissions for the user in the guild (excludes overwrites)
    pub permissions: Option<String>,

    /// voice region id for the guild (deprecated)
    pub region: Option<String>,

    /// id of afk channel
    pub afk_channel_id: Option<String>,

    /// afk timeout in seconds, can be set to: 60, 300, 900, 1800, 3600
    pub afk_timeout: u16,

    /// true if the server widget is enabled
    pub widget_enabled: Option<bool>,

    /// the channel id that the widget will generate an invite to, or null if set to no invite
    pub widget_channel_id: Option<String>,

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
    pub application_id: Option<String>,

    /// the id of the channel where guild notices such as welcome messages and boost events are posted
    pub system_channel_id: Option<String>,

    /// system channel flags
    pub system_channel_flags: SystemChannelFlags,

    /// the id of the channel where Community guilds can display rules and/or guidelines
    pub rules_channel_id: Option<String>,

    /// the maximum number of presences for the guild (null is always returned, apart from the largest of guilds)
    pub max_presences: Option<i128>,

    /// the maximum number of members for the guild
    pub max_members: Option<i128>,

    /// the vanity url code for the guild
    pub vanity_url_code: Option<String>,

    /// the description of a guild
    pub description: Option<String>,

    /// banner hash
    pub banner: Option<String>,

    /// premium tier (Server Boost level)
    pub premium_tier: PremiumTier,

    /// the number of boosts this guild currently has
    pub premium_subscription_count: Option<i128>,

    /// the preferred locale of a Community guild; used in server discovery and notices from Discord, and sent in interactions; defaults to "en-US"
    pub preferred_locale: String,

    /// the id of the channel where admins and moderators of Community guilds receive notices from Discord
    pub public_updates_channel_id: Option<String>,

    /// the maximum amount of users in a video channel
    pub max_video_channel_users: Option<i128>,

    /// approximate number of members in this guild, returned from the GET /guilds/<id> endpoint when with_counts is true
    pub approximate_member_count: Option<i128>,

    /// approximate number of non-offline members in this guild, returned from the GET /guilds/<id> endpoint when with_counts is true
    pub approximate_presence_count: Option<i128>,

    /// the welcome screen of a Community guild, shown to new members, returned in an Invite's guild object
    pub welcome_screen: Option<WelcomeScreen>,

    /// guild NSFW level
    pub nsfw_level: NSFWLevel,

    /// custom guild stickers
    pub stickers: Vec<Sticker>,

    /// whether the guild has the boost progress bar enabled
    pub premium_progress_bar_enabled: bool,
}
