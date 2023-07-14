use serde::Deserialize;
use crate::Snowflake;

#[derive(Deserialize, Debug)]
pub struct UserPartial {
    /// the user's id
    pub id: Snowflake,
}
