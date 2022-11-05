use serde::{Serialize, Deserialize};
use serde_yaml::Error;
use crate::{publisher::Publisher, object::DockerObject};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub logging: String,
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
}

impl Configuration {
    pub fn new(content: &str) -> Result<Configuration, Error>{
        serde_yaml::from_str(content)
    }
    pub fn get_log_level(&self) -> &str{
        &self.settings.logging
    }
}
