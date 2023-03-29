use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct InstallParams {
    pub scopes: Vec<String>,
    pub permissions: String,
}
