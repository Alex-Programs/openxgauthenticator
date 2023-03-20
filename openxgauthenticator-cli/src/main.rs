use confy::ConfyError;
use serde::{Serialize, Deserialize};
use libopenxg;
extern crate rpassword;

use ansi_term::Colour::{Red};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    username: String,
    password: String,
    url: String,
    keepalive_delay: u64,
    retry_delay: u64,
    do_stealth_ua: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            username: "".to_string(),
            password: "".to_string(),
            url: "https://172.29.39.130:8090".to_string(),
            keepalive_delay: 90,
            retry_delay: 5,
            do_stealth_ua: false,
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
        if env::consts::OS == "windows".to_string() {
            println!("Credentials entered here will be stored IN PLAINTEXT in this directory.");
        } else {
            println!("Credentials entered here will be stored {} in this directory.", Red.bold().underline().paint("in plaintext"));
        }

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

        println!("Confirm Password:");
        let confirm_password = rpassword::read_password()
            .expect("Failed to read password");

        if password != confirm_password {
            println!("Passwords do not match. Please try again.");
            std::process::exit(1);
        }

        config.username = username.trim().to_string();
        config.password = password.trim().to_string();

        confy::store_path("config.toml", &config)
            .expect("Failed to store credentials");

        println!("Credentials saved.");

        if env::consts::OS == "windows".to_string() {
            println!("To change credentials in the future, modify the file 'config.toml'");
        } else {
            println!("To change credentials in the future, modify the file {}.", Red.bold().underline().paint("'config.toml'"));
        }
    }

    handle_login(&config);
}

fn handle_login(config: &Config) {
    let client = libopenxg::generate_client();

    loop {
        loop {
            let result = libopenxg::login(&config.url, &config.username, &config.password, &build_ua(config), &client);
            match result {
                Ok(()) => {
                    println!("Success logging in!");
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

        println!("Beginning keepalive loop in {} secs", config.keepalive_delay);
        std::thread::sleep(std::time::Duration::from_secs(config.keepalive_delay));

        loop {
            let result = libopenxg::keepalive(&config.url, &config.username, &build_ua(config), &client);
            match result {
                Ok(()) => {
                    println!("Success! Sleeping for {} seconds before redoing keepalive.", config.keepalive_delay);
                    std::thread::sleep(std::time::Duration::from_secs(config.keepalive_delay));

                    fail_count = 0;
                }
                Err(err) => {
                    println!("{}", err);

                    fail_count += 1;

                    if fail_count > 2 {
                        println!("Too many failures. Retrying login.");

                        fail_count = 0;
                        break;
                    }
                    println!("Failure! Sleeping for {} seconds before retrying keepalive. Failure count: {}/2", config.retry_delay, fail_count);
                    std::thread::sleep(std::time::Duration::from_secs(config.retry_delay));
                }
            }
        }
    }
}

fn build_ua(config: &Config) -> String {
    if config.do_stealth_ua {
        return "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.113 Safari/537.36".to_string();
    }

    "openxgauthenticator-cli/1.1.0 ".to_string() + libopenxg::DEFAULT_UA_SUFFIX
}