use std::fs;
use toml::Value;

pub struct AppConfig {
    pub ip_addr: String,
    pub path_to_names: String,
}

impl AppConfig {
    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string(file_path)?;
        let parsed_toml: Value = toml::from_str(&config_content)?;

        let variables = &parsed_toml["goop_config"];

        Ok(AppConfig {
            ip_addr: variables["ip_addr"].as_str().unwrap().to_string(),
            path_to_names: variables["path_to_names"].as_str().unwrap().to_string(),
        })
    }
}
