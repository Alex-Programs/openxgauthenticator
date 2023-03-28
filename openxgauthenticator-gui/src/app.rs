use crate::config::Config;
use catppuccin_egui;
use crate::update_thread;

pub struct OpenXGApp {
    config: Config,
}

impl OpenXGApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Theme
        catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::FRAPPE);

        // Pull in config file
        let config: Config = match crate::config::load_config() {
            Ok(config) => config,
            Err(_) => Config::default(),
        };

        // TODO icon
        update_thread::start_update_thread(&config);

        Self {
            config,
        }
    }
}

impl eframe::App for OpenXGApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            ref mut config,
        } = self;

        // Minimum FPS (1)
        ctx.request_repaint_after(std::time::Duration::from_millis(1000));

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                if ui.button("Quit").clicked() {
                    _frame.close();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("OpenXGAuthenticator GUI");

            ui.separator();

            ui.heading("Status");
            ui.label(update_thread::CURRENT_STATUS.lock().unwrap().to_string());

            ui.separator();

            ui.label("Address");

            let url_edit = egui::TextEdit::singleline(&mut config.url)
                .hint_text("https://address:port");

            ui.add(url_edit);

            ui.separator();

            ui.label("Credentials");

            let username_edit = egui::TextEdit::singleline(&mut config.username)
                .hint_text("Username");

            ui.add(username_edit);

            let password_edit = egui::TextEdit::singleline(&mut config.password)
                .hint_text("Password")
                .password(true);

            ui.add(password_edit);

            ui.separator();

            ui.label("User Agent");

            let user_agent_edit = egui::TextEdit::singleline(&mut config.user_agent)
                .hint_text("User Agent");

            ui.add(user_agent_edit);

            ui.checkbox(&mut config.set_ua_most_common, "Dynamic Stealth: Update UA to most common");

            if ui.button("Reset UA to default").clicked() {
                config.user_agent = "OpenXGAuthenticator GUI ".to_string() + libopenxg::DEFAULT_UA_SUFFIX;
            }

            ui.separator();

            ui.label("Keep alive interval (seconds):");
            ui.add(egui::Slider::new(&mut config.keepalive_delay, 10..=120));

            ui.label("Retry interval on fail (seconds):");
            ui.add(egui::Slider::new(&mut config.retry_delay, 1..=30));

            ui.separator();

            if ui.button("Save").clicked() {
                // Create thread to save config
                crate::config::save_config(config).unwrap();
                update_thread::SHARED_UPDATE_THREAD_STATE.lock().unwrap().clone_from(config);
            }

            // TODO option to start at startup

            egui::warn_if_debug_build(ui);
        });
    }
}
