use serde::{Serialize, Deserialize};
use shiplift::rep::Event;
use tera::{Tera, Context};

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
    pub fn parse(&self, docker_event: &DockerEvent, event: &Event) -> String{
        let mut context = Context::new();
        if self.name == "container"{
            context.insert("id", &event.actor.id);
            context.insert("container",
                       event.actor.attributes.get("name").unwrap());
            context.insert("image",
                       event.actor.attributes.get("image").unwrap());
        }else if self.name == "network"{
            context.insert("id", &event.actor.id);
            context.insert("network",
                       event.actor.attributes.get("name").unwrap());
            context.insert("type",
                       event.actor.attributes.get("type").unwrap());
        }
        process_message(&docker_event.message, &context)
    }
}

fn process_message(message: &str, context: &Context) -> String {
    let mut ter = Tera::default();
    ter.add_raw_template("message", message).unwrap();
    ter.render("message", context).unwrap()
}
