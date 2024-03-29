use super::{guild::Guild, team::Team, user::User};
use crate::Snowflake;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum Flags {
    /// Indicates if an app uses the Auto Moderation API
    APPLICATION_AUTO_MODERATION_RULE_CREATE_BADGE = 1 << 6,

    /// Intent required for bots in 100 or more servers to receive presence_update events
    GATEWAY_PRESENCE = 1 << 12,

    /// Intent required for bots in under 100 servers to receive presence_update events, found in Bot Settings
    GATEWAY_PRESENCE_LIMITED = 1 << 13,

    /// Intent required for bots in 100 or more servers to receive member-related events like guild_member_add. See list of member-related events under GUILD_MEMBERS
    GATEWAY_GUILD_MEMBERS = 1 << 14,

    /// Intent required for bots in under 100 servers to receive member-related events like guild_member_add, found in Bot Settings. See list of member-related events under GUILD_MEMBERS
    GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15,

    /// Indicates unusual growth of an app that prevents verification
    VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16,

    /// Indicates if an app is embedded within the Discord client (currently unavailable publicly)
    EMBEDDED = 1 << 17,

    /// Intent required for bots in 100 or more servers to receive message content
    GATEWAY_MESSAGE_CONTENT = 1 << 18,

    /// Intent required for bots in under 100 servers to receive message content, found in Bot Settings
    GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19,

    /// Indicates if an app has registered global application commands
    APPLICATION_COMMAND_BADGE = 1 << 23,

    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InstallParams {
    /// the scopes to add the application to the server with
    pub scopes: Vec<String>,

    /// the permissions to request for the bot role
    pub permissions: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Application {
    /// the id of the app
    pub id: Snowflake,

    /// the name of the app
    pub name: String,

    /// the icon hash of the app
    pub icon: Option<String>,

    /// the description of the app
    pub description: String,

    /// an array of rpc origin urls, if rpc is enabled
    pub rpc_origins: Option<Vec<String>>,

    /// when false only app owner can join the app's bot to guilds
    pub bot_public: bool,

    /// when true the app's bot will only join upon completion of the full oauth2 code grant flow
    pub bot_require_code_grant: bool,

    /// the url of the app's terms of service
    pub terms_of_service_url: Option<String>,

    /// the url of the app's privacy policy
    pub privacy_policy_url: Option<String>,

    /// partial user object containing info on the owner of the application
    pub owner: Option<User>,

    #[deprecated]
    /// deprecated and will be removed in v11. An empty string.
    pub summary: String,

    /// the hex encoded key for verification in interactions and the GameSDK's GetTicket
    pub verify_key: String,

    /// if the application belongs to a team, this will be a list of the members of that team
    pub team: Option<Team>,

    /// guild associated with the app. For example, a developer support server.
    pub guild_id: Option<Snowflake>,

    /// a partial object of the associated guild
    pub guild: Option<Guild>,

    /// if this application is a game sold on Discord, this field will be the id of the "Game SKU" that is created, if exists
    pub primary_sku_id: Option<Snowflake>,

    /// if this application is a game sold on Discord, this field will be the URL slug that links to the store page
    pub slug: Option<String>,

    /// the application's default rich presence invite cover image hash
    pub cover_image: Option<String>,

    /// the application's public flags
    pub flags: Option<Flags>,

    /// an approximate count of the app's guild membership.
    pub approximate_guild_count: Option<u32>,

    /// up to 5 tags describing the content and functionality of the application
    pub tags: Option<Vec<String>>,

    /// settings for the application's default in-app authorization link, if enabled
    pub install_params: Option<InstallParams>,

    /// the application's default custom authorization link, if enabled
    pub custom_install_url: Option<String>,

    /// the application's role connection verification entry point, which when configured will render the app as a verification method in the guild role verification configuration
    pub role_connections_verification_url: Option<String>,
}
