use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VoiceRegion {
    /// unique ID for the region
    pub id: String,

    /// name of the region
    pub name: String,

    /// true for a single server that is closest to the current user's client
    pub optimal: bool,

    /// whether this is a deprecated voice region (avoid switching to these)
    pub deprecated: bool,

    /// whether this is a custom voice region (used for events/etc)
    pub custom: bool,
}
