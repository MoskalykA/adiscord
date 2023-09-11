use crate::Snowflake;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserPartial {
    /// the user's id
    pub id: Snowflake,
}
