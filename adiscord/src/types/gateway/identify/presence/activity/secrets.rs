use serde::Serialize;

/*join?	string	Secret for joining a party
spectate?	string	Secret for spectating a game
match?	string	Secret for a specific instanced match */
#[derive(Serialize)]
pub struct GatewayIdentifyPresenceActivitySecrets {
    /// Secret for joining a party
    pub join: Option<String>,

    /// Secret for spectating a game
    pub spectate: Option<String>,

    /// Secret for a specific instanced match
    pub r#match: Option<String>,
}
