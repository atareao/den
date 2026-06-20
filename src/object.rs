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

#[cfg(test)]
mod object_tests {
    use super::*;
    use std::collections::HashMap;
    use docker_api::models::{EventMessage, EventActor};

    fn make_container_object() -> DockerObject {
        DockerObject {
            name: "container".to_string(),
            monitorize: true,
            events: vec![DockerEvent {
                name: "start".to_string(),
                message: "{{container}} started on {{hostname}}".to_string(),
            }],
        }
    }

    fn make_network_object() -> DockerObject {
        DockerObject {
            name: "network".to_string(),
            monitorize: true,
            events: vec![DockerEvent {
                name: "create".to_string(),
                message: "Network {{network}} created on {{hostname}}".to_string(),
            }],
        }
    }

    fn make_volume_object() -> DockerObject {
        DockerObject {
            name: "volume".to_string(),
            monitorize: true,
            events: vec![DockerEvent {
                name: "create".to_string(),
                message: "Volume {{volume}} created with {{driver}}".to_string(),
            }],
        }
    }

    fn make_event_message(
        type_: &str,
        action: &str,
        attrs: Vec<(&str, &str)>,
        id: &str,
        time: i64,
    ) -> EventMessage {
        let attributes: HashMap<String, String> = attrs
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        EventMessage {
            type_: Some(type_.to_string()),
            action: Some(action.to_string()),
            actor: Some(EventActor {
                id: Some(id.to_string()),
                attributes: Some(attributes),
            }),
            time: Some(time),
            time_nano: None,
            scope: None,
        }
    }

    #[test]
    fn get_event_returns_some_when_found() {
        let obj = make_container_object();
        let event = obj.get_event("start");
        assert!(event.is_some());
        assert_eq!(event.unwrap().name, "start");
    }

    #[test]
    fn get_event_returns_none_when_not_found() {
        let obj = make_container_object();
        assert!(obj.get_event("destroy").is_none());
    }

    #[test]
    fn parse_renders_container_template_with_all_variables() {
        let obj = make_container_object();
        let docker_event = DockerEvent {
            name: "start".to_string(),
            message: "{{container}} started on {{hostname}}".to_string(),
        };
        let event = make_event_message(
            "container",
            "start",
            vec![("name", "my-nginx"), ("image", "nginx:latest")],
            "abc123",
            1234567890,
        );
        let result = obj.parse(&docker_event, event, "myserver", true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "my-nginx started on myserver");
    }

    #[test]
    fn parse_renders_network_template_with_all_variables() {
        let obj = make_network_object();
        let docker_event = DockerEvent {
            name: "create".to_string(),
            message: "Network {{network}} created on {{hostname}}".to_string(),
        };
        let event = make_event_message(
            "network",
            "create",
            vec![("type", "bridge"), ("name", "my-network")],
            "abc123",
            1234567890,
        );
        let result = obj.parse(&docker_event, event, "myserver", true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Network my-network created on myserver");
    }

    #[test]
    fn parse_renders_volume_template_with_all_variables() {
        let obj = make_volume_object();
        let docker_event = DockerEvent {
            name: "create".to_string(),
            message: "Volume {{volume}} created with {{driver}}".to_string(),
        };
        let event = make_event_message(
            "volume",
            "create",
            vec![("driver", "local"), ("name", "my-volume")],
            "abc123",
            1234567890,
        );
        let result = obj.parse(&docker_event, event, "myserver", true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Volume my-volume created with local");
    }

    #[test]
    fn parse_returns_error_for_unknown_object_type() {
        let obj = DockerObject {
            name: "unknown".to_string(),
            monitorize: true,
            events: vec![DockerEvent {
                name: "test".to_string(),
                message: "test".to_string(),
            }],
        };
        let docker_event = DockerEvent {
            name: "test".to_string(),
            message: "test".to_string(),
        };
        let event = make_event_message("unknown", "test", vec![("name", "test")], "abc123", 0);
        let result = obj.parse(&docker_event, event, "host", true);
        assert!(result.is_err());
    }

    #[test]
    fn parse_uses_id_as_name_when_name_attribute_missing() {
        let obj = make_container_object();
        let docker_event = DockerEvent {
            name: "start".to_string(),
            message: "{{container}}".to_string(),
        };
        let event = make_event_message(
            "container",
            "start",
            vec![("image", "nginx:latest")],
            "abc123def456",
            1234567890,
        );
        let result = obj.parse(&docker_event, event, "myserver", true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "abc123def456");
    }

    #[test]
    fn parse_uses_datetimeformat_filter() {
        let obj = DockerObject {
            name: "container".to_string(),
            monitorize: true,
            events: vec![DockerEvent {
                name: "start".to_string(),
                message: "{{ timestamp|datetimeformat(format='iso') }}".to_string(),
            }],
        };
        let docker_event = DockerEvent {
            name: "start".to_string(),
            message: "{{ timestamp|datetimeformat(format='iso') }}".to_string(),
        };
        let event = make_event_message(
            "container",
            "start",
            vec![("name", "test"), ("image", "test:latest")],
            "abc123",
            1715000000,
        );
        let result = obj.parse(&docker_event, event, "host", true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "2024-05-06T12:53:20+00:00");
    }
}
