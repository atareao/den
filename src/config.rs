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

#[cfg(test)]
mod config_tests {
    use super::*;

    const VALID_YAML: &str = r#"
settings:
  monitorize_always: true
  docker_uri: unix:///var/run/docker.sock
objects:
  - name: container
    monitorize: true
    events:
      - name: start
        message: "{{container}} started"
  - name: volume
    monitorize: false
    events: []
publishers:
  - service: telegram
    enabled: false
    config:
      token: test
      chat_id: test
"#;

    #[test]
    fn new_returns_configuration_when_valid_yaml() {
        let config = Configuration::new(VALID_YAML).unwrap();
        assert!(config.settings.monitorize_always);
        assert_eq!(config.settings.docker_uri, "unix:///var/run/docker.sock");
    }

    #[test]
    fn new_returns_error_when_invalid_yaml() {
        let result = Configuration::new("invalid: [yaml: broken");
        assert!(result.is_err());
    }

    #[test]
    fn get_object_returns_some_when_found_and_monitorized() {
        let config = Configuration::new(VALID_YAML).unwrap();
        let obj = config.get_object("container");
        assert!(obj.is_some());
        assert_eq!(obj.unwrap().name, "container");
    }

    #[test]
    fn get_object_returns_none_when_not_found() {
        let config = Configuration::new(VALID_YAML).unwrap();
        assert!(config.get_object("nonexistent").is_none());
    }

    #[test]
    fn get_object_returns_none_when_not_monitorized() {
        let config = Configuration::new(VALID_YAML).unwrap();
        assert!(config.get_object("volume").is_none());
    }

    #[test]
    fn get_docker_uri_returns_configured_uri() {
        let config = Configuration::new(VALID_YAML).unwrap();
        assert_eq!(config.get_docker_uri(), "unix:///var/run/docker.sock");
    }

    #[test]
    fn get_docker_uri_returns_default_when_not_set() {
        let yaml = r#"
settings:
  monitorize_always: false
objects: []
publishers: []
"#;
        let config = Configuration::new(yaml).unwrap();
        assert_eq!(config.get_docker_uri(), "/var/run/docker.sock");
    }

    #[test]
    fn is_monitorize_always_returns_true_when_set() {
        let config = Configuration::new(VALID_YAML).unwrap();
        assert!(config.is_monitorize_always());
    }

    #[test]
    fn is_monitorize_always_returns_false_when_unset() {
        let yaml = r#"
settings:
  monitorize_always: false
objects: []
publishers: []
"#;
        let config = Configuration::new(yaml).unwrap();
        assert!(!config.is_monitorize_always());
    }
}
