use self_update_danger_no_ssl_verify as self_update;

fn do_self_update() {
    log("Starting self update...".to_string());

    let status = self_update::backends::github::Update::configure()
        .repo_owner("Alex-Programs")
        .repo_name("openxgauthenticator")
        .bin_name("openxgauthenticator-gui")
        .current_version(self_update::cargo_crate_version!())
        .show_download_progress(true)
        .no_confirm(true)
        .build();

    match status {
        Ok(status) => {
            match status.update() {
                Ok(_) => {
                    log("Update successful".to_string());
                }
                Err(e) => {
                    log(format!("Error updating: {}", e));
                }
            }
        }
        Err(e) => {
            log(format!("Error before update: {}", e));
        }
    }
}

fn log(message: String) {
    println!("{}", message);
    let mut status_edit = crate::app::AUTO_UPDATE_STATUS.lock().unwrap();
    status_edit.clone_from(&message);
}

pub fn self_update_thread() {
    std::thread::spawn(|| {
        do_self_update();
    });
}