use serde_derive::Deserialize;
use std::path::{PathBuf};
use std::{fs, fs::File, io::Read};
use toml::{map::Map, Value};

#[derive(Debug, Deserialize)]
struct Config {
    key: Option<String>,
}

fn get_path() -> PathBuf {
    let mut config_path = PathBuf::from(std::env::current_exe().unwrap());
    config_path.pop();
    config_path.push("config.toml");
    return config_path;
}

pub fn read_config() -> Result<String, String> {
    let mut file = match File::open(get_path()) {
        Ok(f) => f,
        Err(_) => return Err(String::from("")),
    };
    let mut file_str = String::new();
    match file.read_to_string(&mut file_str) {
        Ok(_) => {}
        Err(_) => return Err(String::from("error reading file")),
    };
    let config: Config = toml::from_str(&file_str).unwrap();
    if let Some(key) = config.key {
        Ok(key)
    } else {
        Err(String::from("key not exists"))
    }
}

pub fn save_config(key: &str) -> Result<(), String> {
    let mut config = Map::new();
    config.insert("key".into(), Value::String(String::from(key)));
    let value = Value::Table(config);
    let toml_string = match toml::to_string(&value) {
        Ok(s) => s,
        Err(e) => return Err(format!("saving key with error {}", e)),
    };
    match fs::write(get_path(), toml_string) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("writing to toml with error {}", e)),
    }
}
