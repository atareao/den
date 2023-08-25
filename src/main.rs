mod object;
mod publisher;
mod config;

use futures::StreamExt;
use shiplift::{Docker, rep::Event};
use tokio::fs;
use log::{error, debug, info};
use env_logger::{
    Builder,
    Env
};
use std::process;
use config::Configuration;

#[tokio::main]
async fn main() {
    let content = match fs::read_to_string("config.yml")
        .await {
            Ok(value) => value,
            Err(e) => {
                println!("Error with config file `config.yml`: {}",
                    e.to_string());
                process::exit(0);
            }
        };
    let configuration = Configuration::new(&content)
        .expect("Someting went wrong");
    let hostname = match gethostname::gethostname().to_str() {
        Some(value) => value.to_owned(),
        None => "".to_owned(),
    };
    Builder::from_env(
        Env::default()
            .default_filter_or(configuration.get_log_level())).init();
    info!("Configuration loaded");

    let docker = Docker::new();
    info!("Start listening for events");

    while let Some(event_result) = docker.events(&Default::default()).next().await {
        match event_result {
            Ok(event) => {
                process(event, &configuration, &hostname).await;
            },
            Err(e) => error!("Error: {}", e),
        };
    }
    info!("End listening for events");
}

async fn process(event: Event, config: &Configuration, hostname: &str){
    debug!("event => {:?}", event);
    match config.get_object(&event.typ){
        Some(docker_object) => {
            match docker_object.get_event(&event.action) {
                Some(docker_event) => {
                    match docker_object.parse(&docker_event, event, hostname) {
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
                            debug!("Error: {}", &e);
                        },
                    };
                },
                None => {},
            }
        },
        None => {},
    }
}
