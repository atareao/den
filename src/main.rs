mod object;
mod publisher;
mod config;
mod filters;
mod error;

use futures::StreamExt;
use docker_api::{Docker, models::EventMessage, opts::EventsOptsBuilder};
use tokio::fs;
use tracing::{debug, error, info};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter
};
use std::process;
use config::Configuration;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();


    let content = fs::read_to_string("config.yml")
        .await
        .expect("Error with config file");
    let configuration = Configuration::new(&content)
        .expect("Someting went wrong");
    let hostname = match gethostname::gethostname().to_str() {
        Some(value) => value.to_owned(),
        None => "".to_owned(),
    };

    info!("Configuration loaded");

    let uri = configuration.get_docker_uri();
    let docker = match Docker::new(uri){
        Ok(docker) => docker,
        Err(e) => {
            error!("Can't init docker: {}", e);
            process::exit(1);
        },
    };
    info!("Start listening for events");

    while let Some(event_result) = docker.events(&EventsOptsBuilder::default().build()).next().await {
        match event_result {
            Ok(event) => {
                process(event, &configuration, &hostname).await;
            },
            Err(e) => error!("Error: {}", e),
        };
    }
    info!("End listening for events");
}

async fn process(event: EventMessage, config: &Configuration, hostname: &str){
    debug!("event => {:?}", event);
    let actor = event.actor.clone();
    let monitorize = match actor
            .unwrap()
            .attributes
            .unwrap()
            .get("es.atareao.den.monitorize"){
        Some(value) => *value == "true",
        None => config.is_monitorize_always(),
    };
    let type_ = &event.type_.as_ref().unwrap();
     if let Some(docker_object) = config.get_object(type_){
        let action = &event.action.clone();
        if let Some(docker_event) = docker_object.get_event(action.as_ref().unwrap()){
            match docker_object.parse(&docker_event, event.clone(), hostname,
                    monitorize) {
                Ok(message) => {
                    debug!("============================");
                    debug!("Object: {}", docker_object.name);
                    debug!("Event: {}", docker_event.name);
                    debug!("Message: {}", &message);
                    for publisher in config.publishers.iter(){
                        if publisher.enabled{
                            match publisher.post_message(&message).await{
                                Ok(response) => info!("Send: {:?}", response),
                                Err(e) => error!("Error in sending: {:?}", e),
                            };
                        }
                    }
                },
                Err(e) => {
                    error!("Error: {}", &e);
                },
            };
            }
    }
}
