use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use reqwest::{Client, Response, header::{HeaderMap, HeaderValue,
    HeaderName}};
use std::str::FromStr;
use std::collections::HashMap;
use anyhow::{Error, anyhow};
use log::{error, debug, info};
use urlencoding::encode;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Publisher{
    pub service: String,
    pub enabled: bool,
    pub config: HashMap<String, String>,
}

const MATTERMOST: &str = "mattermost";
const TELEGRAM: &str = "telegram";
const ZINC: &str = "zinc";
const MATRIX: &str = "matrix";


impl Publisher{

    pub async fn post_message(&self, message: &str) -> Result<Response, Error>{
        if self.service.to_lowercase() == MATTERMOST {
            return Ok(self.post_with_mattermost(message).await?);
        }else if self.service.to_lowercase() == TELEGRAM {
            return Ok(self.post_with_telegram(message).await?);
        }else if self.service.to_lowercase() == ZINC {
            return Ok(self.post_with_zinc(message).await?);
        }else if self.service.to_lowercase() == MATRIX {
            return Ok(self.post_with_matrix(message).await?);
        }else{
            error!("Publisher not defined: {}", self.service);
            Err(anyhow!("Publisher not defined: {}", self.service))
        }
    }

    async fn post_with_zinc(&self, message: &str) -> Result<Response, reqwest::Error>{
        debug!("Post with zinc: {}", message);
        let base_url = self.config.get("url").unwrap();
        let index = self.config.get("index").unwrap();
        let token = self.config.get("token").unwrap();
        let url = format!("{}/api/default/{}/_json", base_url, index);
        info!("Url: {}", url);
        info!("Message: {}", message);
        let mut header_map = HeaderMap::new();
        header_map.append(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.append(HeaderName::from_str("Accept").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.append(HeaderName::from_str("Authorization").unwrap(),
                          HeaderValue::from_str(&format!("Basic {}", token)).unwrap());
        info!("header_map: {:?}", &header_map);
        let body = json!([{
            "message": message,
        }]);
        Self::post(&url, header_map, &body).await
    }

    async fn post_with_telegram(&self, message: &str) -> Result<Response, reqwest::Error>{
        debug!("Post with telegram: {}", message);
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

    async fn post_with_matrix(&self, message: &str) -> Result<Response, reqwest::Error>{
        debug!("Post with matrix: {}", message);
        let token = self.config.get("token").unwrap();
        let room = encode(self.config.get("room").unwrap());
        let base_url = self.config.get("url").unwrap();
        let uuid = Uuid::new_v4().to_string();
        let url = format!("https://{}/_matrix/client/v3/rooms/{}:{}/send/m.room.message/{}", base_url, room, base_url, uuid);
        let body = json!({
                "body": message,
                "msgtype": "m.text",
        });
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.append(HeaderName::from_str("Authorization").unwrap(),
                          HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());
        Self::put(&url, header_map, &body).await
    }

    async fn post_with_mattermost(&self, message: &str) -> Result<Response, reqwest::Error>{
        debug!("Post with mattermost: {}", message);
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
    async fn put(url: &str, header_map: HeaderMap, body: &Value) -> Result<Response, reqwest::Error>{
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        let content = serde_json::to_string(body).unwrap();
        client.put(url).body(content).send().await
    }
}

