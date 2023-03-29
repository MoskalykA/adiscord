pub mod flags;
pub mod install_params;

use self::{flags::ApplicationFlags, install_params::InstallParams};
use super::{team::Team, user::User};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub rpc_origins: Option<Vec<String>>,
    pub bot_publi: bool,
    pub bot_require_code_grant: bool,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub owner: Option<User>,

    #[deprecated]
    pub summary: String,

    pub verify_key: String,
    pub team: Option<Team>,
    pub guild_id: Option<String>,
    pub primary_sku_id: Option<String>,
    pub slug: Option<String>,
    pub cover_image: Option<String>,
    pub flags: Option<ApplicationFlags>,
    pub tags: Option<Vec<String>>,
    pub install_params: Option<InstallParams>,
    pub custom_install_url: Option<String>,
    pub role_connections_verification_url: Option<String>,
}
