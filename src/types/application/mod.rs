pub mod flags;
pub mod install_params;

use self::{flags::ApplicationFlags, install_params::InstallParams};
use super::{team::Team, user::User};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Application {
    /// the id of the app
    pub id: String,

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

    /// if this application is a game sold on Discord, this field will be the guild to which it has been linked
    pub guild_id: Option<String>,

    /// if this application is a game sold on Discord, this field will be the id of the "Game SKU" that is created, if exists
    pub primary_sku_id: Option<String>,

    /// if this application is a game sold on Discord, this field will be the URL slug that links to the store page
    pub slug: Option<String>,

    /// the application's default rich presence invite cover image hash
    pub cover_image: Option<String>,

    /// the application's public flags
    pub flags: Option<ApplicationFlags>,

    /// up to 5 tags describing the content and functionality of the application
    pub tags: Option<Vec<String>>,

    /// settings for the application's default in-app authorization link, if enabled
    pub install_params: Option<InstallParams>,

    /// the application's default custom authorization link, if enabled
    pub custom_install_url: Option<String>,

    /// the application's role connection verification entry point, which when configured will render the app as a verification method in the guild role verification configuration
    pub role_connections_verification_url: Option<String>,
}
