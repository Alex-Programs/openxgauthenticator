use confy::ConfyError;
use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use reqwest;
use reqwest::blocking::Client;
use std::collections::hash_map::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    username: String,
    password: String,
    url: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Config {
            username: "".to_string(),
            password: "".to_string(),
            url: "http://127.0.0.1:8090".to_string(), // TODO change before release
        }
    }
}

fn main() {
    let credentials: Result<Config, ConfyError> = confy::load_path("config.toml");

    let mut config: Config = match credentials {
        Ok(credentials) => credentials,
        Err(err) => Config::default(),
    };

    if config.username == "" || config.password == "" {
        println!("No credentials found. Please enter your credentials.");

        let mut username = String::new();
        let mut password = String::new();

        while username == "" {
            println!("Username:");
            std::io::stdin().read_line(&mut username)
                .expect("Failed to read line");
        }

        while password == "" {
            println!("Password:");
            std::io::stdin().read_line(&mut password)
                .expect("Failed to read line");
        }

        config.username = username.trim().to_string();
        config.password = password.trim().to_string();

        confy::store_path("config.toml", &config)
            .expect("Failed to store credentials");

        println!("Credentials saved.");
        println!("To change credentials in the future, modify the file 'config.toml'.");
    }

    handle_login(&config);
}

fn handle_login(config: &Config) {
    println!("Handling login...");

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create client");

    login(&config, &client);
}

fn login(config: &Config, client: &Client) -> Result<(), String> {
    println!("Logging in...");

    // build request
    let a: u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let a: &str = &a.to_string();
    let mut data = HashMap::new();

    data.insert("mode", "191");
    data.insert("a", a);
    data.insert("producttype", "0");
    data.insert("username", &config.username);
    data.insert("password", &config.password);

    let response = client.post(format!("{}/login.xml", &config.url).as_str())
        .form(&data)
        .send();

    match response {
        Ok(response) => {
            if response.status() != reqwest::StatusCode::OK {
                println!("Login failed. Status code: {}", response.status());
                return Err("Invalid status code".to_owned());
            }
        }
        Err(err) => {
            println!("Failed to login: {}", err);

            return Err(err.to_string());
        }
    }

    println!("Login successful.");

    Ok(())
}

// TODO:
// Keepalive func
// actually loop etc
// ensure ctrl+c works as intended
// switch over hosts
// test
// release