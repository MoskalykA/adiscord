use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserPartial {
    /// the user's id
    pub id: String,
}
