use serde::{Deserialize, Serialize};
use confy;
use dirs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub user_agent: String,
    pub set_ua_most_common: bool,
    pub last_ua_most_common: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub keepalive_delay: u64,
    pub retry_delay: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            user_agent: "OpenXGAuthenticator GUI ".to_string() + libopenxg::DEFAULT_UA_SUFFIX,
            set_ua_most_common: false,
            last_ua_most_common: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            url: "https://172.29.39.130:8090".to_string(),
            keepalive_delay: 90,
            retry_delay: 5,
        }
    }
}

fn get_config_path() -> String {
    let mut path = dirs::config_dir().unwrap();
    path.push("openxg-gui.toml");
    path.as_os_str().to_str().unwrap().to_string()
}

pub fn load_config() -> Result<Config, confy::ConfyError> {
    confy::load_path(get_config_path())
}

pub fn save_config(config: &Config) -> Result<(), confy::ConfyError> {
    confy::store_path(get_config_path(), config)
}