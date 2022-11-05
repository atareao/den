use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DockerEvent<'a>{
    name: &'a str,
    message: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DockerObject<'a>{
    name: &'a str,
    monitorize: bool,
    events: Vec<DockerEvent<'a>>,
}
