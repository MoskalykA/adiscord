use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum Opcode {
    /// Receive An event was dispatched.
    Dispatch,

    /// Send/Receive Fired periodically by the client to keep the connection alive.
    Heartbeat,

    /// Send Start a new session during the initial handshake.
    Identify,

    /// Send Update the client's presence.
    PresenceUpdate,

    /// Send Used to join/leave or move between voice channels.
    VoiceStateUpdate,

    /// Send Resume a previous session that was disconnected.
    Resume = 6,

    /// Receive You should attempt to reconnect and resume immediately.
    Reconnect,

    /// Send Request information about offline guild members in a large guild.
    RequestGuildMembers,

    /// Receive The session has been invalidated. You should reconnect and identify/resume accordingly.
    InvalidSession,

    /// Receive Sent immediately after connecting, contains the heartbeat_interval to use.
    Hello,

    /// Receive Sent in response to receiving a heartbeat to acknowledge that it has been received.
    HeartbeatAck,
}
