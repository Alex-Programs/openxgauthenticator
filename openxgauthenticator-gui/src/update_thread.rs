use libopenxg;
use std::sync::Mutex;
use crate::config::Config;
use std::time::SystemTime;
use std::thread::sleep;
use once_cell::sync::Lazy;

pub static SHARED_UPDATE_THREAD_STATE: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::default()));
pub static CURRENT_STATUS: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("No Status Data".to_string()));

pub fn start_update_thread(config: &Config) {
    SHARED_UPDATE_THREAD_STATE.lock().unwrap().clone_from(config);

    std::thread::spawn(|| {
        let mut client = libopenxg::generate_client();

        let mut are_logged_in = false;
        let mut last_login_fail_time: i64 = 0;
        let mut last_login_error = "".to_string();
        let mut last_login_keepalive_error_type = "Login".to_string();
        let mut last_keepalive_login_time: i64 = 0;

        loop {
            sleep(std::time::Duration::from_secs(1));

            let current_state = SHARED_UPDATE_THREAD_STATE.lock().unwrap().clone();

            println!("{:?}", current_state);

            if are_logged_in {
                // TODO pull UA

                let time_until_keepalive: i64 = last_keepalive_login_time + (current_state.keepalive_delay as i64) - (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64);

                if time_until_keepalive > 0 {
                    set_status(format!("Logged in ({}s keepalive)", time_until_keepalive));
                } else {
                    set_status("Attempting to keepalive".to_string());

                    let keepalive_result = libopenxg::keepalive(&current_state.url, &current_state.username, &current_state.user_agent, &client);

                    match keepalive_result {
                        Ok(_) => {
                            set_status("Keepalive successful".to_string());
                            last_keepalive_login_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
                        },
                        Err(e) => {
                            set_status(format!("Keepalive error: {}", e));
                            are_logged_in = false;
                            last_login_fail_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
                            last_login_error = e.to_string();
                            last_login_keepalive_error_type = "Keepalive".to_string();
                        }
                    }
                }
            } else {
                let time_until_retry = last_login_fail_time + (current_state.retry_delay as i64) - (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64);

                if time_until_retry > 0 {
                    set_status(format!("{} error: {}, ({}s retry)", last_login_keepalive_error_type, last_login_error, time_until_retry));
                } else {
                    set_status("Attempting to login".to_string());

                    let login_result = libopenxg::login(&current_state.url, &current_state.username, &current_state.password, &current_state.user_agent, &client);
    
                    match login_result {
                        Ok(_) => {
                            set_status("Logged in".to_string());
                            last_keepalive_login_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
                            are_logged_in = true;
                        },
                        Err(e) => {
                            set_status(format!("Login error: {}", e));
                            last_login_error = e.to_string();
                            last_login_fail_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
                            last_login_keepalive_error_type = "Login".to_string();
                            last_keepalive_login_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;

                            client = libopenxg::generate_client(); // Reset the client to maybe fix the issue
                        }
                    }
                }
            }
        }
    });
}

fn set_status(status: String) {
    CURRENT_STATUS.lock().unwrap().clone_from(&status);
    println!("{}", status);
}