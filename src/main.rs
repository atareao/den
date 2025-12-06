mod config;
mod error;
mod filters;
mod object;
mod publisher;

use config::Configuration;
use docker_api::{
    models::EventMessage, opts::{ContainerFilter, ContainerListOpts, ContainerStatus, EventsOptsBuilder}, Container, Docker
};
use futures::StreamExt;
use serde::Deserialize;
use std::process;
use tokio::fs;
use tracing::{debug, error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Para las estadísticas de un contenedor: /containers/{id}/stats
#[derive(Deserialize, Debug)]
struct CpuUsage {
    total_usage: u64,
}

#[derive(Deserialize, Debug)]
struct CpuStats {
    cpu_usage: CpuUsage,
    system_cpu_usage: u64,
    online_cpus: u32,
}

#[derive(Deserialize, Debug)]
struct ContainerStats {
    precpu_stats: CpuStats,
    cpu_stats: CpuStats,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let content = fs::read_to_string("config.yml")
        .await
        .expect("Error with config file");
    let configuration = Configuration::new(&content).expect("Someting went wrong");
    let hostname = match gethostname::gethostname().to_str() {
        Some(value) => value.to_owned(),
        None => "".to_owned(),
    };

    info!("Configuration loaded");

    let uri = configuration.get_docker_uri();
    let docker = match Docker::new(uri) {
        Ok(docker) => docker,
        Err(e) => {
            error!("Can't init docker: {}", e);
            process::exit(1);
        }
    };
    info!("Start listening for events");

    let docker_clone = docker.clone();
    tokio::task::spawn(async {
        let opts = ContainerListOpts::builder()
            .filter([ContainerFilter::Status(ContainerStatus::Running)])
            .build();
        let containers = docker_api::Containers::new(docker_clone);
        loop {
            for summary in containers.list(&opts).await.unwrap_or_default().as_slice() {
                let id = summary.id.as_ref().unwrap();
                let name = summary
                    .names
                    .as_ref()
                    .unwrap_or(&vec![])
                    .first()
                    .unwrap_or(&"unknown".to_string())
                    .replace("/", "");
                let container = containers.get(id);
                // `stats()` devuelve un Stream de estadísticas. Solo queremos la primera.
                if let Some(Ok(stats)) = container.stats().next().await {
                    let cpu_stats_usage = stats.get("cpu_stats")
                        .and_then(|s| s.get("cpu_usage"))
                        .unwrap_or(&serde_json::json!(0.0)).as_f64().unwrap_or(0.0);
                    let precpu_stats_usage = stats.get("precpu_stats")
                        .and_then(|s| s.get("cpu_usage"))
                        .unwrap_or(&serde_json::json!(0.0)).as_f64().unwrap_or(0.0);
                    let system_cpu_usage = stats.get("cpu_stats")
                        .and_then(|s| s.get("system_cpu_usage"))
                        .unwrap_or(&serde_json::json!(0.0)).as_f64().unwrap_or(0.0);
                    let pre_system_cpu_usage = stats.get("precpu_stats")
                        .and_then(|s| s.get("system_cpu_usage"))
                        .unwrap_or(&serde_json::json!(0.0)).as_f64().unwrap_or(0.0);
                    let online_cpus = stats.get("cpu_stats")
                        .and_then(|s| s.get("online_cpus"))
                        .unwrap_or(&serde_json::json!(0.0)).as_f64().unwrap_or(0.0);
                    let cpu_delta = cpu_stats_usage - precpu_stats_usage;
                    let system_delta = system_cpu_usage - pre_system_cpu_usage;
                    let number_of_cores = online_cpus;
                    let mut cpu_percentage = 0.0;
                    if system_delta > 0.0 && cpu_delta > 0.0 {
                        cpu_percentage = (cpu_delta / system_delta) * number_of_cores * 100.0;
                    }
                    let short_id = &id[..12];
                    println!("{:<15} {:<30} {:.2}%", short_id, name, cpu_percentage);
                }
                debug!("Container: {} ({})", name, id);
            }
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });
    debug!("start");

    while let Some(event_result) = docker
        .events(&EventsOptsBuilder::default().build())
        .next()
        .await
    {
        match event_result {
            Ok(event) => {
                process(event, &configuration, &hostname).await;
            }
            Err(e) => error!("Error: {}", e),
        };
    }
    info!("End listening for events");
}

async fn process(event: EventMessage, config: &Configuration, hostname: &str) {
    debug!("event => {:?}", event);
    let actor = event.actor.clone();
    let monitorize = match actor
        .unwrap()
        .attributes
        .unwrap()
        .get("es.atareao.den.monitorize")
    {
        Some(value) => *value == "true",
        None => config.is_monitorize_always(),
    };
    let type_ = &event.type_.as_ref().unwrap();
    if let Some(docker_object) = config.get_object(type_) {
        let action = &event.action.clone();
        if let Some(docker_event) = docker_object.get_event(action.as_ref().unwrap()) {
            match docker_object.parse(&docker_event, event.clone(), hostname, monitorize) {
                Ok(message) => {
                    debug!("============================");
                    debug!("Object: {}", docker_object.name);
                    debug!("Event: {}", docker_event.name);
                    debug!("Message: {}", &message);
                    for publisher in config.publishers.iter() {
                        if publisher.enabled {
                            match publisher.post_message(&message).await {
                                Ok(response) => info!("Send: {:?}", response),
                                Err(e) => error!("Error in sending: {:?}", e),
                            };
                        }
                    }
                }
                Err(e) => {
                    error!("Error: {}", &e);
                }
            };
        }
    }
}
