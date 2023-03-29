pub mod member;

use self::member::TeamMember;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Team {
    /// a hash of the image of the team's icon
    pub icon: Option<String>,

    /// the unique id of the team
    pub id: String,

    /// the members of the team
    pub members: Vec<TeamMember>,

    /// the name of the team
    pub name: String,

    /// the user id of the current team owner
    pub owner_user_id: String,
}
