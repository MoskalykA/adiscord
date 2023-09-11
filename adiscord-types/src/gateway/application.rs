//use crate::api:::application::flags::ApplicationFlags;
use crate::Snowflake;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Partial {
    /// the id of the app
    pub id: Snowflake,
    /*/// the application's public flags
    pub flags: Option<ApplicationFlags>,*/
}
