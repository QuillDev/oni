use std::borrow::Borrow;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Request, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::api::discord_data::message::Message;

//The client for making our various requests
pub struct OniClient {
    token: String,
    request_client: reqwest::Client,
}


impl OniClient {
    //Construct a new client
    pub fn new(token: &str, request_client: reqwest::Client) -> OniClient {
        return OniClient { token: token.to_string(), request_client };
    }

    //Make an authenticated post request
    async fn auth_post<P: Serialize + ?Sized, R: DeserializeOwned>(&self, url: &str, payload: &P) -> Result<R, reqwest::Error> {
        let headers = self.create_auth_headers();
        let response = self.request_client.post(url)
            .headers(headers)
            .json(payload.borrow())
            .send()
            .await?
            .json::<R>()
            .await?;

        Ok(response)
    }

    //Make an authenticated get request
    async fn auth_get<T: DeserializeOwned>(&self, url: &str) -> Result<T, reqwest::Error> {
        let headers = self.create_auth_headers();
        let response = self.request_client.get(url)
            .headers(headers)
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(response)
    }

    //Send a message to the channel
    pub async fn send_message(&self, channel: &str, content: &str) -> Result<Message, reqwest::Error> {
        let base_url = format!("https://discordapp.com/api/channels/{}/messages", channel);

        //Insert data
        let mut data = HashMap::new();
        data.insert("content", content);

        let response = self.auth_post(base_url.as_str(), &data).await?;


        // let message = client
        //     .post(base_url)
        //     .headers(headers)
        //     .json(&data)
        //     .send()
        //     .await?;

        Ok(response)
    }

    fn create_auth_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        //Auth Header
        headers.insert(
            AUTHORIZATION,
            format!("Bot {}", &self.token).parse().unwrap(),
        );
        //Content Header
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        return headers;
    }
}

