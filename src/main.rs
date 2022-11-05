mod object;
mod publisher;
mod config;

use futures::StreamExt;
use shiplift::{Docker, rep::Event};
use tokio::fs;
use log::{error, info};
use env_logger::Builder;
use std::process;
use config::Configuration;
//use crossbeam::channel::{unbounded, Receiver, Sender};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};

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
    info!("Configuration loaded");
    Builder::new()
        .filter_level(match configuration.get_log_level(){
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => log::LevelFilter::Off,
        })
        .parse_default_env()
        .init();


    info!("listening for events");

    //let (sender, receiver): (Sender<Event>, Receiver<Event>) = unbounded();
    let (sender, mut receiver): (UnboundedSender<Event>, UnboundedReceiver<Event>) = unbounded_channel();

    tokio::spawn(async move {
        loop{
            match receiver.recv().await{
                Some(event) => {
                    info!("{:?}", event);
                    process(&event, &configuration).await;
                },
                None => error!(""),
            }
        }
    });
    let docker = Docker::new();

    while let Some(event_result) = docker.events(&Default::default()).next().await {
        match event_result {
            Ok(event) => {
                info!("event -> {:?}", event);
                sender.send(event).unwrap()
            },
            Err(e) => error!("Error: {}", e),
        };
    }
}

async fn process(event: &Event, config: &Configuration){
    // if docker_object in monitorized_docker_objects &&
    //     event in docker_object.events
    info!("event => {:?}", event);
    match config.get_object(&event.typ){
        Some(docker_object) => {
            match docker_object.get_event(&event.action) {
                Some(docker_event) => {
                    let message = docker_object.parse(&docker_event, event);
                    info!("============================");
                    info!("Object: {}", docker_object.name);
                    info!("Event: {}", docker_event.name);
                    info!("Message: {}", &message);
                    for publisher in config.publishers.iter(){
                        if publisher.enabled{
                            match publisher.post_message(&message).await{
                                Ok(response) => info!("Send: {:?}", response),
                                Err(e) => error!("Error in sending: {:?}", e),
                            };
                        }
                    }
                    /*
                    config.publishers.iter().for_each(closure!(clone publisher, ||{
                        //tokio::spawn(async {
                        //    match publisher.post_message(&message).await{
                        //        Ok(response) => info!("Send: {:?}", response),
                        //        Err(e) => error!("Error in sending: {:?}", e),
                        //    }
                        //});
                    }));
                    for publisher in config.publishers.iter_mut(){
                        tokio::spawn(async move {
                            match publisher.post_message(&message_to).await{
                                Ok(response) => info!("Send: {:?}", response),
                                Err(e) => error!("Error in sending: {:?}", e),
                            }
                        });
                    }
                    */
                },
                None => {},
            }
        },
        None => {},
    }
}
