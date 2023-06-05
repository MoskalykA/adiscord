# adiscord

An API and Gateway Discord wrapper in Rust

## Installation

Use the package manager [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to install adiscord.

### API installation

```bash
cargo add adiscord
cargo add tokio --features rt-multi-thread
```

### Gateway installation

```bash
cargo add adiscord --features gateway
cargo add adiscord_intents
cargo add tokio --features rt-multi-thread
```

## Usage

(These examples require dotenv to be installed in order to have .env files. Here is the command `cargo add dotenv`)

### Example with the API

```rust
use adiscord::Client;
use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() {
    // Client initialisation
    let mut client = Client::new("10", dotenv!("TOKEN"));

    // Recover a server from its ID
    match client.guild.get("1089521338286342195").await {
        Ok(guild) => println!("{:?}", guild.name),
        Err(error) => println!("{:?}", error),
    };
}
```

### Example with the Gateway

```rust
use adiscord::Client;
use adiscord_intents::Intent;
use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() {
    // Client initialisation
    let mut client = Client::new("10", dotenv!("TOKEN"));

    // Receive heartbeat messages, delete this line if you do not wish to do so
    client.set_heartbeat_ack(true);

    // Send all the permissions you need from your bot to discord
    client.add_intents(vec![
        Intent::Guilds,
        Intent::GuildMembers,
        Intent::GuildModeration,
        Intent::GuildEmojisAndStickers,
        Intent::GuildIntegrations,
        Intent::GuildWebhooks,
        Intent::GuildInvites,
        Intent::GuildVoiceStates,
        Intent::GuildPresences,
        Intent::GuildMessages,
        Intent::GuildMessageReactions,
        Intent::GuildMessageTyping,
        Intent::DirectMessages,
        Intent::DirectMessageReactions,
        Intent::DirectMessageTyping,
        Intent::MessageContent,
        Intent::GuildScheduledEvents,
        Intent::AutoModerationConfiguration,
        Intent::AutoModerationExecution,
    ]);

    // The ready event will be launched once your bot is connected
    client.on_ready(|ready| {
        println!("{:?}", ready);
    });
}
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)
