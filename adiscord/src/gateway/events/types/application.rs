//use crate::types::application::flags::ApplicationFlags;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApplicationPartial {
    /// the id of the app
    pub id: String,
    /*/// the application's public flags
    pub flags: Option<ApplicationFlags>,*/
}
