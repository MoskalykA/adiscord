use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Status {
    /// User's status set for an active desktop (Windows, Linux, Mac) application session
    pub desktop: Option<String>,

    /// User's status set for an active mobile (iOS, Android) application session
    pub mobile: Option<String>,

    /// User's status set for an active web (browser, bot user) application session
    pub web: Option<String>,
}
