use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Overwrite {
    /// role or user id
    pub id: String,

    /// either 0 (role) or 1 (member)
    pub r#type: u8,

    /// permission bit set
    pub allow: String,

    /// permission bit set
    pub deny: String,
}
