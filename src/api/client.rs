use std::collections::HashMap;

    use crate::api::{self, message::Message};
    use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};

    pub struct OniClient {
        token: String,
    }

    impl OniClient {
        //Construct a new client
        pub fn new(token: &str) -> OniClient {
            return OniClient {
                token: token.to_string(),
            };
        }

        //Send a message to the channel
        pub async fn send_message(&self, channel: &str, content: &str) -> Result<Message, reqwest::Error> {
            let client = reqwest::Client::new();

            //Create auth headers
            let mut headers = HeaderMap::new();
            headers.insert(
                AUTHORIZATION,
                ("Bot ".to_string() + &self.token).parse().unwrap(),
            );
            headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

            //Intert data
            let mut data = HashMap::new();
            data.insert("content", content);

            let base_url = format!("https://discordapp.com/api/channels/{}/messages", channel);
            let message = client
                .post(base_url)
                .headers(headers)
                .json(&data)
                .send()
                .await?;

            let message = message.json::<api::message::Message>().await?;
            Ok(message)
        }
    }