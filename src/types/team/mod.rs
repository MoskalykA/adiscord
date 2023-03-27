pub mod member;

use self::member::TeamMember;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Team {
    pub icon: Option<String>,
    pub id: String,
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: String,
}
