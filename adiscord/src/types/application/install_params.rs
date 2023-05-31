use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct InstallParams {
    /// the scopes to add the application to the server with
    pub scopes: Vec<String>,

    /// the permissions to request for the bot role
    pub permissions: String,
}
