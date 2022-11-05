use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use reqwest::{Client, Response, header::{HeaderMap, HeaderValue,
    HeaderName}};
use std::str::FromStr;
use std::collections::HashMap;
use anyhow::{Error, anyhow};

#[derive(Debug, Serialize, Deserialize)]
pub struct Publisher<'a>{
    service: &'a str,
    enabled: bool,
    config: HashMap<&'a str, &'a str>,
}

const MATTERMOST: &str = "mattermost";
const TELEGRAM: &str = "telegram";


impl<'a> Publisher<'a>{
    pub fn new(enabled: bool, service: &'a str, config: HashMap<&'a str, &'a str>) -> Publisher<'a>{
        Self{
            service,
            enabled,
            config,
        }
    }

    pub async fn post_message(&self, message: &str) -> Result<Response, Error>{
        if self.service.to_lowercase() == MATTERMOST{
            return Ok(self.post_with_mattermost(message).await?);
        }else if self.service.to_lowercase() == TELEGRAM{
            return Ok(self.post_with_telegram(message).await?);
        }else{
            Err(anyhow!("Publisher not defined: {}", self.service))
        }
    }

    async fn post_with_telegram(&self, message: &str) -> Result<Response, reqwest::Error>{
        let token = self.config.get("token").unwrap();
        let chat_id = self.config.get("chat_id").unwrap();
        let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
        let body = json!({
                "chat_id": chat_id,
                "text": message,
        });
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        Self::post(&url, header_map, &body).await
    }


    async fn post_with_mattermost(&self, message: &str)->Result<Response, reqwest::Error>{
        let url = format!("{}/api/v4/posts", self.config.get("url").unwrap());
        let token = self.config.get("token").unwrap();
        let channel_id = self.config.get("channel_id").unwrap();
        let body = json!({
                "channel_id": channel_id,
                "message": message,
        });
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.insert(HeaderName::from_str("Authorization").unwrap(),
                          HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());
        Self::post(&url, header_map, &body).await
    }

    async fn post(url: &str, header_map: HeaderMap, body: &Value) -> Result<Response, reqwest::Error>{
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        let content = serde_json::to_string(body).unwrap();
        client.post(url).body(content).send().await
    }
}

