use serde::{Serialize, Deserialize};
use serde_yaml::Error;
use crate::{publisher::Publisher, object::DockerObject};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub monitorize_always: bool,
    #[serde(default = "get_default_docker_uri")]
    docker_uri: String,
}
fn get_default_docker_uri() -> String{
    String::from("/var/run/docker.sock")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub settings: Settings,
    pub objects: Vec<DockerObject>,
    pub publishers: Vec<Publisher>,
}

impl Configuration {
    pub fn get_object(&self, name: &str) -> Option<DockerObject>{
        for docker_object in self.objects.iter(){
            if docker_object.name == name && docker_object.monitorize{
                return Some(docker_object.clone());
            }
        }
        None
    }
    pub fn get_docker_uri(&self) -> &str{
        &self.settings.docker_uri
    }
}

impl Configuration {
    pub fn new(content: &str) -> Result<Configuration, Error>{
        serde_yaml::from_str(content)
    }
    pub fn is_monitorize_always(&self) -> bool{
        self.settings.monitorize_always
    }
}
