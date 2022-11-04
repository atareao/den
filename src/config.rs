use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Configuration {
    settings: Settings,
    events: Events,
    integrations: Integrations,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    logging: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DockerObject {
    monitorize: bool,
    events: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Events {
    container: DockerObject,
    image: DockerObject,
    plugin: DockerObject,
    volume: DockerObject,
    network: DockerObject,
    daemon: DockerObject,
    service: DockerObject,
    node: DockerObject,
    secret: DockerObject,
    config: DockerObject,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Integration {
    enabled: bool,
    config: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Integrations {
    slack: Integration,
    discord: Integration,
    mattermost: Integration,
    telegram: Integration,
}


