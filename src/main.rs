mod api;

use std::process::exit;

use api::client::OniClient;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let token = std::env::var("TOKEN").expect("You must supply a token!");

    //Testing the client
    let client = OniClient::new(&token, reqwest::Client::new());
    let message = client
        .send_message("825526127065169930", "wow this is cool!")
        .await?;

    let author = message.author;
    println!("The author of that message was {}", author.username);
    println!("Their avatar is {}", author.avatar);
    Ok(())
}