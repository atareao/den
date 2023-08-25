use chrono::Local;

use serde::{Serialize, Deserialize};
use shiplift::rep::Event;
use minijinja::{
    Environment,
    context,
};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DockerEvent{
    pub name: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DockerObject{
    pub name: String,
    pub monitorize: bool,
    pub events: Vec<DockerEvent>,
}

impl DockerObject {
    pub fn get_event(&self, name: &str) -> Option<DockerEvent>{
        for docker_event in self.events.iter(){
            if docker_event.name == name{
                return Some(docker_event.clone());
            }
        }
        None
    }
    pub fn parse(&self, docker_event: &DockerEvent, event: Event,
            hostname: &str) -> Result<String, minijinja::Error>{
        let mut env = Environment::new();
        let template = env.template_from_str(&docker_event.message).unwrap();
        let name = match event.actor.attributes.get("name"){
            Some(name) => name.clone(),
            None => event.actor.id.clone(),
        };
        if self.name == "container"{
            let ctx = context! {
                hostname  => hostname.to_string(),
                timestamp => Local::now().timestamp().to_string(),
                id        => event.actor.id.to_string(),
                name      => name.to_string(),
                image     => event.actor.attributes.get("image").unwrap(),
            };
            template.render(ctx)
        }else if self.name == "network"{
            let ctx = context! {
                hostname  => hostname.to_string(),
                timestamp => Local::now().timestamp().to_string(),
                id        => event.actor.id.to_string(),
                name      => name.to_string(),
                type      => event.actor.attributes.get("type").unwrap(),
            };
            template.render(ctx)
        }else if self.name == "volume"{
            let ctx = context! {
                hostname  => hostname.to_string(),
                timestamp => Local::now().timestamp().to_string(),
                id        => event.actor.id.to_string(),
                name      => name.to_string(),
                driver    => event.actor.attributes.get("driver").unwrap(),
            };
            template.render(ctx)
        }else{
            Err(minijinja::Error::new(minijinja::ErrorKind::NonKey, "Error"))
        }
    }
}
