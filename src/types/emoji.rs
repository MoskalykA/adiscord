use super::{role::Role, user::User};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Emoji {
    pub id: String,
    pub name: String,
    pub roles: Vec<Role>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}
