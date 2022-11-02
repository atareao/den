use serde_json::{json, Value};
use reqwest::{Client, Response, Error, header::{HeaderMap, HeaderValue,
    HeaderName}};
use std::str::FromStr;


pub struct Telegram{
    url: String,
    token: String,
    channel_id: String,
}

impl Telegram{
    pub fn new(url: &str, token: &str, channel_id: &str) -> Telegram{
        Self {
            url: url.to_string(),
            token: token.to_string(),
            channel_id: channel_id.to_string(),
        }
    }

    pub async fn post_message(&self, message: &str) -> Result<Response, Error>{
        let url = format!("https://api.telegram.org/bot{}/sendMessage",
            self.token);
        let body = json!({
                "chat_id": self.channel_id,
                "text": message,
        });
        self.post(&url, Some(body)).await
    }

    async fn post(&self, url: &str, body: Option<Value>)->Result<Response, Error>{
        println!("URL: {}", url);
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        match body{
            Some(value) => {
                let content = serde_json::to_string(&value).unwrap();
                let res = client.post(url).body(content).send().await?;
                Ok(res)
            },
            None => {
                let res = client.post(url).send().await?;
                Ok(res)
            },
        }
    }
}
