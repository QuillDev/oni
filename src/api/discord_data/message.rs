use serde::Deserialize;

mod message {}

#[derive(Deserialize)]
pub struct Message {
    pub id: String,
    pub content: String,
    pub channel_id: String,
    pub author: Author,
}

#[derive(Deserialize)]
pub struct Author {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub discriminator: String,
    pub public_flags: i16,
    pub bot: bool,
}
