use adiscord::Client;
use adiscord_intents::Intent;
use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() {
    let mut client = Client::new("10", dotenv!("TOKEN"), adiscord::TokenType::Bot);
    client.set_heartbeat_ack(true);

    client.add_intent(Intent::MESSAGE_CONTENT);
    client.add_intent(Intent::GUILDS);
    client.add_intent(Intent::GUILD_MESSAGES);
    client.add_intent(Intent::GUILD_MEMBERS);

    // Message

    client.on_message(|message| {
        println!("Create message -> {}", message.content);
    });

    client.on_message_update(|message| {
        println!("Update message -> {}", message.content);
    });

    // Channel

    client.on_channel_create(|channel| {
        println!("Create chanel -> {:?}", channel.name);
    });

    client.on_channel_update(|channel| {
        println!("Update chanel -> {:?}", channel.name);
    });

    client.on_channel_delete(|channel| {
        println!("Delete chanel -> {:?}", channel.name);
    });

    // Thread

    client.on_thread_create(|channel| {
        println!("Create thread -> {:?}", channel.name);
    });

    client.on_thread_update(|channel| {
        println!("Update thread -> {:?}", channel.name);
    });

    client.on_thread_delete(|channel| {
        println!("Delete thread -> {:?}", channel.name);
    });

    client.init().await;
}
