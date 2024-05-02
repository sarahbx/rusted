use std::collections::HashMap;
use std::path::Path;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::value::{Value};

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub env: HashMap<String, Value>,
    pub local: Option<HashMap<String, Value>>,
    pub auth: Option<HashMap<String, HashMap<String, Value>>>,
    pub hosts: Option<HashMap<String, HashMap<String, Value>>>,
}

impl Config {
    pub fn new(yaml_file_path: &str) -> Box<Self> {
        assert!(Path::new(yaml_file_path).exists(), "Config file not found");

        Box::new(config::Config::builder()
            .add_source(config::File::new(yaml_file_path, config::FileFormat::Toml).required(true))
            .add_source(config::Environment::default())
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap())
    }

    pub fn get_string_from_hashmap(map: &HashMap<String, Value>, key: &str) -> Box<String> {
        let value = map.get(key).unwrap();
        let value_as_string = value.to_string();
        return if value_as_string.starts_with('"') {
            Box::new(serde_json::from_value(value.clone()).expect("Unable to deserialize"))
        } else {
            Box::new(value_as_string)
        }
    }

    pub fn get_host_config(&self, cluster_name: &str) ->  Box<HashMap<String, Value>> {
        for (_, host_config) in self.hosts.as_ref().unwrap() {
            let regex = Self::get_string_from_hashmap(&host_config, "regex");
            let compiled_regex = Regex::new(regex.as_str()).expect(format!("Regex {:?} did not compile", regex).as_str());
            if compiled_regex.is_match(cluster_name) {
                return Box::new(host_config.clone())
            }
        }
        Box::new(HashMap::new())
    }

    pub fn get_auth_config(&self, host_config: &Box<HashMap<String, Value>>) -> Box<HashMap<String, Value>> {
        let host_auth = Self::get_string_from_hashmap(&host_config,"auth");
        for (auth_name, auth_config) in self.auth.as_ref().unwrap() {
            if auth_name.as_str() == host_auth.as_str() {
                return Box::new(auth_config.clone())
            }
        }
        Box::new(HashMap::new())
    }

    pub fn get_hostname(&self, host_config: &Box<HashMap<String, Value>>, auth_config: &Box<HashMap<String, Value>>, cluster_name: &str) -> Box<String> {
        if !host_config.is_empty() {
            if host_config.get("hostname_format").is_some() {
                let hostname_format = Self::get_string_from_hashmap(&host_config, "hostname_format");
                return if hostname_format.contains("{cluster_name}") {
                    Box::new(hostname_format.replace("{cluster_name}", cluster_name))
                } else {
                    hostname_format
                }
            } else {
                Box::new(auth_config.get("host").unwrap().to_string())
            }
        }
        else {
            Box::new("".to_string())
        }
    }

    pub fn get_auth(&self, auth_config: &Box<HashMap<String, Value>>, key: &str) -> Box<String> {
        Box::new(auth_config.get(key).unwrap().to_string())
    }

    pub fn get_ssh_key(&self, auth_config: &Box<HashMap<String, Value>>) -> Box<String> {
        let ssh_key = Box::new(auth_config.get("ssh_key").unwrap().to_string());
        self.populate_local_home(ssh_key)
    }

    pub fn get_remote_cluster_directory(&self, host_config: &Box<HashMap<String, Value>>) -> Box<String> {
        if !host_config.is_empty() {
            let cluster_directory = Self::get_string_from_hashmap(&host_config, "cluster_directory");
            cluster_directory
        } else {
            Box::new("".to_string())
        }

    }

    /* Private */
    fn populate_local_home(&self, boxed_string: Box<String>) -> Box<String> {
        return if boxed_string.contains("{HOME}") {
            let home_dir = self.env.get("home").unwrap().as_str().unwrap();
            Box::new(boxed_string.replace("{HOME}", home_dir))
        } else {
            boxed_string
        }
    }
}
