use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use reqwest::{Client, Response, header::{HeaderMap, HeaderValue,
    HeaderName}};
use std::str::FromStr;
use std::collections::HashMap;
use log::{error, debug, info};
use urlencoding::encode;
use std::time::{SystemTime, UNIX_EPOCH};
use std::time::Duration;
use amiquip::{Connection, Exchange, Publish, Result};
use super::error::CustomError;
use mqtt_async_client::client::{
    Client as MQTTClient,
    Publish as PublishOpts,
    KeepAlive,
    QoS::ExactlyOnce
};

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
const MOSQUITTO: &str = "mosquitto";
const RABBITMQ: &str = "rabbitmq";


impl Publisher{

    pub async fn post_message(&self, message: &str) -> Result<String, CustomError>{
        if self.service.to_lowercase() == MATTERMOST {
            return Ok(self.post_with_mattermost(message).await?);
        }else if self.service.to_lowercase() == TELEGRAM {
            return Ok(self.post_with_telegram(message).await?);
        }else if self.service.to_lowercase() == ZINC {
            return Ok(self.post_with_zinc(message).await?);
        }else if self.service.to_lowercase() == MATRIX {
            return Ok(self.post_with_matrix(message).await?);
        }else if self.service.to_lowercase() == MOSQUITTO {
            //return Ok(self.post_with_mqtt(message).await?);
            error!("Publisher not implementd right now: {}", self.service);
            Err(CustomError::new(format!("Publisher not implementd right now: {}", self.service)))
        }else if self.service.to_lowercase() == RABBITMQ {
            return Ok(self.post_with_rabbitmq(message).await?);
        }else{
            error!("Publisher not defined: {}", self.service);
            Err(CustomError::new(format!("Publisher not defined: {}", self.service)))
        }
    }

    async fn post_with_rabbitmq(&self, message: &str) -> Result<String, CustomError>{
        debug!("Post with rabbitmq: {}", message);
        // Open connection.
        let host = self.config.get("host").unwrap();
        let port = self.config.get("port").unwrap();
        let user = self.config.get("user").unwrap();
        let password = self.config.get("password").unwrap();
        let queue = self.config.get("queue").unwrap();
        let url = format!("amqp://{}:{}@{}:{}", user, password, host, port);
        let mut connection = Connection::insecure_open(&url)
            .map_err(|err|{
                CustomError::new(err.to_string())
            })?;

        // Open a channel - None says let the library choose the channel ID.
        let channel = connection.open_channel(None)
            .map_err(|err|{
                CustomError::new(err.to_string())
            })?;

        // Get a handle to the direct exchange on our channel.
        let exchange = Exchange::direct(&channel);

        // Publish a message to the "hello" queue.
        exchange.publish(Publish::new(message.as_bytes(), queue))
            .map_err(|err|{
                CustomError::new(err.to_string())
            })?;

        connection.close()
            .map_err(|err|{
                CustomError::new(err.to_string())
            })?;
        Ok(format!("Posted with Rabbitmq: {}", message))
    }

    async fn post_with_mqtt(&self, message: &str) -> Result<String, CustomError>{
        debug!("Post with mqtt: {}", message);
        let now = SystemTime::now();
        let ts = now.duration_since(UNIX_EPOCH).expect("Time went backwrds").as_secs();
        let client_id = ts.to_string();
        let user = match self.config.get("user") {
            Some(username) => Some(username.to_string()),
            None => None,
        };
        let password = self.config.get("password").unwrap();
        let host = self.config.get("host").unwrap();
        let port = self.config.get("port").unwrap();
        let topic = self.config.get("topic").unwrap();
        let url = format!("mqtt://{}:{}", host, port);
        info!("Url: {}", url);
        let mut builder = MQTTClient::builder();
        let mut client = builder.set_url_string(&url)
            .map_err(|err|{
                CustomError::new(err.to_string())
            })?
            .set_username(user)
            .set_password(Some(password.clone().into_bytes()))
            .set_client_id(Some(client_id))
            .set_connect_retry_delay(Duration::from_secs(1))
            .set_keep_alive(KeepAlive::from_secs(1))
            .set_operation_timeout(Duration::from_secs(1))
            .set_automatic_connect(true)
            .build()
            .map_err(|err|{
                CustomError::new(err.to_string())
            })?;
        client.connect()
            .await
            .map_err(|err|{
                CustomError::new(err.to_string())
            })?;
        let mut p = PublishOpts::new(
            topic.clone(), 
            message.as_bytes().to_vec());
        p.set_qos(ExactlyOnce);
        p.set_retain(true);
        client.publish(&p)
            .await
            .map_err(|err|{
                CustomError::new(err.to_string())
            })?;
        client.disconnect()
            .await
            .map_err(|err|{
                CustomError::new(err.to_string())
            })?;
        Ok(message.to_string())
    }

    async fn post_with_zinc(&self, message: &str) -> Result<String, CustomError>{
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
        Self::post(&url, header_map, &body)
            .await.map_err(|err| CustomError::new(err.to_string()))?
            .text()
            .await.map_err(|err| CustomError::new(err.to_string()))

    }

    async fn post_with_telegram(&self, message: &str) -> Result<String, CustomError>{
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
        Self::post(&url, header_map, &body)
            .await.map_err(|err| CustomError::new(err.to_string()))?
            .text()
            .await.map_err(|err| CustomError::new(err.to_string()))
    }

    async fn post_with_matrix(&self, message: &str) -> Result<String, CustomError>{
        debug!("Post with matrix: {}", message);
        let token = self.config.get("token").unwrap();
        let room = encode(self.config.get("room").unwrap());
        let base_url = self.config.get("url").unwrap();
        let now = SystemTime::now();
        let ts = now.duration_since(UNIX_EPOCH).expect("Time went backwrds").as_secs();
        let url = format!("https://{}/_matrix/client/v3/rooms/{}:{}/send/m.room.message/{}", base_url, room, base_url, ts);
        let mut html = markdown::to_html(message);
        html = html[..html.len()-1].to_string();
        let body = json!({
            "msgtype": "m.text",
            "format": "org.matrix.custom.html",
            "body": message,
            "formatted_body": html
        });
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.append(HeaderName::from_str("Authorization").unwrap(),
                          HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());
        Self::put(&url, header_map, &body)
            .await.map_err(|err| CustomError::new(err.to_string()))?
            .text()
            .await.map_err(|err| CustomError::new(err.to_string()))
    }

    async fn post_with_mattermost(&self, message: &str) -> Result<String, CustomError>{
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
        Self::post(&url, header_map, &body)
            .await.map_err(|err| CustomError::new(err.to_string()))?
            .text()
            .await.map_err(|err| CustomError::new(err.to_string()))
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

