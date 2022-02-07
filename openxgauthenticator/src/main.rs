use std::borrow::Borrow;
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
            url: "https://172.29.39.130:8090".to_string(),
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

    loop {
        let mut fail_count = 0;

        loop {
            let result = login(&config, &client);
            match result {
                Ok(()) => {
                    println!("Moving to keepalive loop.");
                    break;
                }
                Err(err) => {
                    println!("{}", err);
                    println!("Sleeping for 5 seconds before retrying login.");
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            }
        }

        loop {
            let result = keepalive(&config, &client);
            match result {
                Ok(()) => {
                    println!("Sleeping for 90 seconds before retrying keepalive.");
                    std::thread::sleep(std::time::Duration::from_secs(90));
                    break;
                }
                Err(err) => {
                    println!("{}", err);

                    fail_count += 1;

                    if fail_count > 3 {
                        println!("Too many failures. Retrying login.");
                        break;
                    }
                    println!("Sleeping for 5 seconds before retrying keepalive. Failure count: {}/3", fail_count);
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            }
        }
    }
}

// the 'a is lifetimes, saying that the references must live as long as the data in hashmap
fn build_req<'a>(username: &'a str, password: &'a str, mode: &'a str) -> HashMap<&'a str, &'a str> {
    let a: u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let a: &str = &a.to_string();

    let mut data = HashMap::new();

    data.insert("mode", mode);
    data.insert("a", a);
    data.insert("producttype", "0");
    data.insert("username", username);

    if mode == "191" {
        data.insert("password", password);
    }

    data
}

fn login(config: &Config, client: &Client) -> Result<(), String> {
    println!("Logging in...");

    let data = build_req(&config.username, &config.password, "191");

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

fn keepalive(config: &Config, client: &Client) -> Result<(), String> {
    println!("Keeping alive...");

    let data = build_req(&config.username, &config.password, "192");

    let response = client.post(format!("{}/live", &config.url).as_str())
        .form(&data)
        .send();

    match response {
        Ok(response) => {
            if response.status() != reqwest::StatusCode::OK {
                println!("Keepalive failed. Status code: {}", response.status());
                return Err("Invalid status code".to_owned());
            }
        }
        Err(err) => {
            println!("Failed to keepalive: {}", err);

            return Err(err.to_string());
        }
    }

    println!("Keepalive successful.");

    Ok(())
}

// TODO:
// make hashmap use String not &str in order to safely return from the creation function without using references
// error handling
// ensure ctrl+c works as intended
// switch over hosts
// test
// release