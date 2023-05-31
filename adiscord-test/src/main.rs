use adiscord::Client;
use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() {
    let mut client = Client::new("10", dotenv!("TOKEN"), adiscord::TokenType::Bot);
    client.on_message(|message| {
        println!("{:?}", message.id);
    });

    client.init().await;
}
