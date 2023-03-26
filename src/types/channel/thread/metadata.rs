use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u16,
    pub archive_timestamp: String,
    pub locked: bool,
    pub invitable: Option<bool>,
    pub create_timestamp: Option<String>,
}
