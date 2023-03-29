use confy;
use dirs;
use serde::{Deserialize, Serialize};
use std::path::Path;
use toml;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub user_agent: String,
    pub set_ua_most_common: bool,
    pub was_last_ua_most_common: bool,
    pub username: String,
    pub password: String,
    pub url: String,
    pub keepalive_delay: u64,
    pub retry_delay: u64,
    pub calc_current_ua: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            user_agent: "OpenXGAuthenticator GUI ".to_string() + libopenxg::DEFAULT_UA_SUFFIX,
            set_ua_most_common: false,
            was_last_ua_most_common: false,
            username: "".to_string(),
            password: "".to_string(),
            url: "https://172.29.39.130:8090".to_string(),
            keepalive_delay: 90,
            retry_delay: 5,
            calc_current_ua: "NO UA".to_string(),
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OldConfig {
    username: String,
    password: String,
    url: String,
    keepalive_delay: u64,
    retry_delay: u64,
    do_stealth_ua: bool,
}


impl Into<Config> for OldConfig {
    fn into(self) -> Config {
        let mut ua = "OpenXGAuthenticator GUI ".to_string() + libopenxg::DEFAULT_UA_SUFFIX;
        if self.do_stealth_ua {
            ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".to_string();
        }

        Config {
            user_agent: ua.clone(),
            set_ua_most_common: self.do_stealth_ua,
            was_last_ua_most_common: self.do_stealth_ua,
            username: self.username,
            password: self.password,
            url: self.url,
            keepalive_delay: self.keepalive_delay,
            retry_delay: self.retry_delay,
            calc_current_ua: ua,
        }
    }
}

pub fn does_old_config_exist() -> bool {
    let old_path = "config.toml".to_string();

    if Path::new(&old_path).exists() {
        let contents = std::fs::read_to_string(&old_path);
        if contents.is_err() {
            return false;
        }

        let old_config: OldConfig = match toml::from_str(&contents.unwrap()) {
            Ok(c) => c,
            Err(_) => return false,
        };

        return true;
    };

    false
}

pub fn load_old_config() -> Result<OldConfig, String> {
    if does_old_config_exist() == false {
        return Err("Old config does not exist".to_string());
    }

    let mut old_path = "config.toml".to_string();

    let contents = std::fs::read_to_string(&old_path).unwrap();

    let old_config: OldConfig = match toml::from_str(&contents) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string()),
    };

    Ok(old_config)
}

pub fn rm_old_config() -> Result<(), std::io::Error> {
    let old_path = "config.toml".to_string();
    std::fs::remove_file(&old_path)
}