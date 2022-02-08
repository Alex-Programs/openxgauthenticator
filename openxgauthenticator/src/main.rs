use confy::ConfyError;
use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use reqwest;
use reqwest::blocking::Client;
use std::collections::hash_map::HashMap;
extern crate rpassword;
use ansi_term::Colour::{Red};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    username: String,
    password: String,
    url: String,
    keepalive_delay: u64,
    retry_delay: u64,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Config {
            username: "".to_string(),
            password: "".to_string(),
            url: "https://172.29.39.130:8090".to_string(),
            keepalive_delay: 90,
            retry_delay: 5,
        }
    }
}

fn main() {
    let credentials: Result<Config, ConfyError> = confy::load_path("config.toml");

    let mut config: Config = match credentials {
        Ok(credentials) => credentials,
        Err(_) => Config::default(),
    };

    if config.username == "" || config.password == "" {
        println!("No credentials found. Please enter your credentials.");
        println!("Credentials entered here will be stored {} in this directory.", Red.bold().underline().paint("in plaintext"));

        let mut username = String::new();
        let mut password = String::new();

        while username == "" {
            println!("Username:");
            std::io::stdin().read_line(&mut username)
                .expect("Failed to read line");
        }

        while password == "" {
            println!("Password:");
            password = rpassword::read_password()
                .expect("Failed to read password");
        }

        config.username = username.trim().to_string();
        config.password = password.trim().to_string();

        confy::store_path("config.toml", &config)
            .expect("Failed to store credentials");

        println!("Credentials saved.");
        println!("To change credentials in the future, modify the file {}.", Red.bold().underline().paint("'config.toml'"));
    }

    handle_login(&config);
}

fn handle_login(config: &Config) {
    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create client");

    loop {
        loop {
            let result = login(&config, &client);
            match result {
                Ok(()) => {
                    println!("Success logging in! Moving to keepalive loop.");
                    break;
                }
                Err(err) => {
                    println!("{}", err);
                    println!("Failure! Sleeping for {} seconds before retrying login.", config.retry_delay);
                    std::thread::sleep(std::time::Duration::from_secs(config.retry_delay));
                }
            }
        }

        let mut fail_count = 0;

        loop {
            let result = keepalive(&config, &client);
            match result {
                Ok(()) => {
                    println!("Success! Sleeping for {} seconds before redoing keepalive.", config.keepalive_delay);
                    std::thread::sleep(std::time::Duration::from_secs(config.keepalive_delay));

                    fail_count = 0;
                }
                Err(err) => {
                    println!("{}", err);

                    fail_count += 1;

                    if fail_count > 3 {
                        println!("Too many failures. Retrying login.");

                        fail_count = 0;
                        break;
                    }
                    println!("Failure! Sleeping for {} seconds before retrying keepalive. Failure count: {}/3", config.retry_delay, fail_count);
                    std::thread::sleep(std::time::Duration::from_secs(config.retry_delay));
                }
            }
        }
    }
}

// the 'a is lifetimes, saying that the references must live as long as the data in hashmap
fn build_req<'a>(username: &'a str, password: &'a str, mode: &'a str) -> HashMap<&'a str, String> {
    let a: u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    let mut data: HashMap<&str, String> = HashMap::new();

    data.insert("mode", mode.to_string());
    data.insert("a", a.to_string());
    data.insert("producttype", "0".to_string());
    data.insert("username", username.to_string());

    if mode == "191" {
        data.insert("password", password.to_string());
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

    Ok(())
}

fn keepalive(config: &Config, client: &Client) -> Result<(), String> {
    println!("Keeping alive...");

    let data = build_req(&config.username, &config.password, "192");

    // yes, get. yes, sophos is that stupid.
    let response = client.get(format!("{}/live", &config.url).as_str())
        // yes params. yes it's inconsistent. yes it's stupid
        .query(&data)
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

    Ok(())
}

// TODO: Dist website