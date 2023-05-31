use serde_repr::Serialize_repr;

#[allow(non_camel_case_types)]
#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum GatewayIdentifyPresenceActivityType {
    Game,
    Streaming,
    Listening,
    Watching,
    Custom,
    Competing,
}
