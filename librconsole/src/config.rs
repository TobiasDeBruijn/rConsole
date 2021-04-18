use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use rand::Rng;
use std::io::Write;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub port:   u32,
    pub pepper: String,
    pub keys:   Vec<KeyItem>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct KeyItem {
    pub key:    String,
    pub name:   String
}

impl Config {

    /**
    Load the configuration from disk
    The provided configuration file directory should exist!

    ## Parameters
        file_path_buf: The absolute path to the configuration file.

    ## Returns
        Ok: The parsed and deserialized Config object
        Err: The Error as a String
    */
    pub fn load(file_path_buf: PathBuf) -> Result<Config, String> {
        let file_path = file_path_buf.as_path();
        if !file_path.exists() {
            let default_config = Config {
                port: 8090,
                pepper: rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(16).map(char::from).collect(),
                keys: vec![KeyItem { key: format!("example_key_{}", rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(8).map(char::from).collect::<String>()), name: "example_name".to_string() }]
            };

            let file = std::fs::File::create(file_path);
            if file.is_err() {
                return Err(file.err().unwrap().to_string());
            }

            let _ = file.unwrap().write_all(serde_yaml::to_string(&default_config).unwrap().as_bytes());
            return Ok(default_config);
        }

        let config_file_content_wrapped = std::fs::read_to_string(file_path);
        if config_file_content_wrapped.is_err() {
            return Err(config_file_content_wrapped.err().unwrap().to_string());
        }

        let config_file_content = config_file_content_wrapped.unwrap();
        let config_file_deserialized= serde_yaml::from_str::<Config>(&config_file_content);
        if config_file_deserialized.is_err() {
            return Err(config_file_deserialized.err().unwrap().to_string());
        }

        Ok(config_file_deserialized.unwrap())
    }
}