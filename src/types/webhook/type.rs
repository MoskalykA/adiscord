use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum WebhookType {
    Incoming = 1,
    Channel = 2,
    Application = 3,
}
