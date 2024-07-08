use serde::{Serialize, Deserialize};
use docker_api::models::EventMessage;
use minijinja::{
    Environment,
    context,
};
use tracing::debug;
use super::filters;


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
    pub fn parse(&self, docker_event: &DockerEvent, event_message: EventMessage,
            hostname: &str, monitorize: bool) -> Result<String, minijinja::Error>{
        let mut env = Environment::new();
        env.add_filter("datetimeformat", filters::datetimeformat);
        let template = env.template_from_str(&docker_event.message).unwrap();
        debug!("Event: {:?}", event_message);
        debug!("Actor: {:?}", event_message.actor);
        let actor = event_message.actor.clone().unwrap();
        let name = match actor.attributes.clone().unwrap().get("name"){
            Some(name) => name.clone(),
            None => actor.id.clone().unwrap().clone(),
        };
        if self.name == "container" && monitorize{
            let ctx = context! {
                hostname  => hostname.to_string(),
                timestamp => event_message.time,
                id        => actor.id.unwrap().to_string(),
                container => name.to_string(),
                image     => actor.attributes.unwrap().get("image").unwrap(),
            };
            template.render(ctx)
        }else if self.name == "network"{
            let ctx = context! {
                hostname  => hostname.to_string(),
                timestamp => event_message.time,
                id        => actor.id.unwrap().to_string(),
                network   => name.to_string(),
                type      => actor.attributes.unwrap().get("type").unwrap(),
            };
            template.render(ctx)
        }else if self.name == "volume"{
            let ctx = context! {
                hostname  => hostname.to_string(),
                timestamp => event_message.time,
                id        => actor.id.unwrap().to_string(),
                volume    => name.to_string(),
                driver    => actor.attributes.unwrap().get("driver").unwrap(),
            };
            template.render(ctx)
        }else{
            Err(minijinja::Error::new(minijinja::ErrorKind::NonKey, "Error"))
        }
    }
}
