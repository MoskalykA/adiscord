use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VoiceRegion {
    pub id: String,
    pub name: String,
    pub optimal: bool,
    pub deprecated: bool,
    pub custom: bool,
}
