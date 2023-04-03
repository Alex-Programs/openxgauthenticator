use crate::config::Config;
use reqwest::blocking::Client;

pub fn get_most_frequent_ua() -> Result<String, String> {
    // Get from API
    let host = "https://openxg.alexcj.co.uk";
    let path = "/most_common_ua.txt";

    let client = Client::builder()
        .user_agent("OpenXGAuthenticator GUI Collecting UA")
        .timeout(std::time::Duration::from_secs(5))
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let req = match client
    .get(format!("{}{}", host, path))
    .send() {
        Ok(res) => res,
        Err(e) => return Err(format!("Error getting most frequent UA: {}", e)),
    };

    let res = match req.text() {
        Ok(res) => res,
        Err(e) => return Err(format!("Error getting most frequent UA: {}", e)),
    };

    Ok(res)
}

pub fn get_current_ua(config: &mut Config) -> String {
    if config.set_ua_most_common {
        if config.was_last_ua_most_common {
            return config.user_agent.clone();
        } else {
            // hardcoded common UA
            return "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".to_string();
        }
    }

    config.was_last_ua_most_common = false;

    config.user_agent.clone()
}