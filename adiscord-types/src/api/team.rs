use super::user::User;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, Debug)]
pub enum MemberState {
    INVITED = 1,
    ACCEPTED,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Member {
    /// the user's membership state on the team
    pub membership_state: MemberState,

    /// will always be ["*"]
    pub permissions: Vec<String>,

    /// the id of the parent team of which they are a member
    pub team_id: String,

    /// the avatar, discriminator, id, and username of the user
    pub user: User,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Team {
    /// a hash of the image of the team's icon
    pub icon: Option<String>,

    /// the unique id of the team
    pub id: String,

    /// the members of the team
    pub members: Vec<Member>,

    /// the name of the team
    pub name: String,

    /// the user id of the current team owner
    pub owner_user_id: String,
}
