mod mattermost;

use futures::StreamExt;
use shiplift::Docker;
use tokio::fs;
use yaml_rust::YamlLoader;

#[tokio::main]
async fn main() {
    let content = fs::read_to_string("config.yml")
        .await
        .expect("config.yml not found");
    let configuration = YamlLoader::load_from_str(&content)
        .expect("Something happened");
    let docker = Docker::new();
    println!("listening for events");

    while let Some(event_result) = docker.events(&Default::default()).next().await {
        match event_result {
            Ok(event) => println!("event -> {:?}", event),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
