use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum WebhookType {
    /// Incoming Webhooks can post messages to channels with a generated token
    Incoming = 1,

    /// Channel Follower Webhooks are internal webhooks used with Channel Following to post new messages into channels
    Channel,

    /// Application webhooks are webhooks used with Interactions
    Application,
}
