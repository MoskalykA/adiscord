use adiscord::Client;
use adiscord_intents::Intent;
use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() {
    let mut client = Client::new("10", dotenv!("TOKEN"), adiscord::TokenType::Bot);
    client.add_intent(Intent::GUILD_MESSAGES);

    client.on_message(|message| {
        println!("{:?}", message.id);
    });

    client.init().await;
}
