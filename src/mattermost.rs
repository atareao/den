use serde_json::{json, Value};
use reqwest::{Client, Response, Error, header::{HeaderMap, HeaderValue,
    HeaderName}};
use std::str::FromStr;


pub struct Mattermost{
    url: String,
    token: String,
    channel_id: String,
}

impl Mattermost{
    pub fn new(url: &str, token: &str, channel_id: &str) -> Mattermost{
        Self {
            url: url.to_string(),
            token: token.to_string(),
            channel_id: channel_id.to_string(),
        }
    }

    pub async fn post_message(&self, message: &str) -> Result<Response, Error>{
        let url = format!("{}/api/v4/posts", self.url);
        let body = json!({
                "channel_id": self.channel_id,
                "message": message,
        });
        self.post(&url, Some(body)).await
    }

    async fn post(&self, url: &str, body: Option<Value>)->Result<Response, Error>{
        println!("URL: {}", url);
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.insert(HeaderName::from_str("Authorization").unwrap(),
                          HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap());
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
