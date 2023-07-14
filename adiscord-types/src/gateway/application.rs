//use crate::api:::application::flags::ApplicationFlags;
use serde::Deserialize;
use crate::Snowflake;

#[derive(Deserialize, Debug)]
pub struct Partial {
    /// the id of the app
    pub id: Snowflake,
    
    /*/// the application's public flags
    pub flags: Option<ApplicationFlags>,*/
}
