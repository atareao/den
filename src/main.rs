mod mattermost;
mod telegram;
mod config;

use futures::StreamExt;
use shiplift::{Docker, rep::Event};
use tokio::fs;
use yaml_rust::{YamlLoader, Yaml};
use log::{error, info};
use env_logger::Builder;
use std::process;

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
    let data = match YamlLoader::load_from_str(&content){
        Ok(value) => value[0]
            .clone()
            .as_hash()
            .expect("someting happened with `config.yml`")
            .clone(),
        Err(e) => {
            println!("Cant read the config file `config.yml`: {}",
                e.to_string());
            process::exit(0);
        }
    };
    let settings = data.get(&Yaml::from_str("settings"))
        .expect("Can't read settings")
        .as_hash()
        .unwrap();


    let log_level = settings.get(&Yaml::from_str("logging"))
        .unwrap()
        .as_str()
        .unwrap_or("info");

    Builder::new()
        .filter_level(match log_level{
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => log::LevelFilter::Off,
        })
        .parse_default_env()
        .init();

    info!("Configuration loaded");

    let docker = Docker::new();
    info!("listening for events");

    while let Some(event_result) = docker.events(&Default::default()).next().await {
        match event_result {
            Ok(event) => println!("event -> {:?}", event),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn process(event: &Event, configuration: &Vec<Yaml>){
    // if docker_object in monitorized_docker_objects &&
    //     event in docker_object.events
}
