use crate::config::Config;
use catppuccin_egui;
use crate::update_thread;
use crate::config;
use crate::ua;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static UA_STATUS: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("No UA Status Data".to_string()));
pub static AUTO_UPDATE_STATUS: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("No Auto Update Status Data".to_string()));

pub struct OpenXGApp {
    config: Config,
    show_old_config_popup: bool,
}

impl OpenXGApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Theme
        catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::FRAPPE);

        // Pull in config file
        let mut config: Config = match crate::config::load_config() {
            Ok(config) => config,
            Err(_) => Config::default(),
        };

        config.calc_current_ua = ua::get_current_ua(&mut config);

        // TODO icon
        update_thread::start_update_thread(&config);
        update_thread::ua_update_thread();

        Self {
            config,
            show_old_config_popup: config::does_old_config_exist(),
        }
    }
}

impl eframe::App for OpenXGApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            ref mut config,
            ref mut show_old_config_popup,
        } = self;

        // Minimum FPS (1)
        ctx.request_repaint_after(std::time::Duration::from_millis(1000));

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if *show_old_config_popup {
            egui::Window::new("Old config detected")
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label("Old config detected. Would you like to import it and delete the old one?");
                    ui.separator();
                    if ui.button("Yes").clicked() {
                        let old_config = crate::config::load_old_config().unwrap();
                        *config = old_config.into();
                        update_thread::SHARED_UPDATE_THREAD_STATE.lock().unwrap().clone_from(config);
                        crate::config::save_config(config).unwrap();
                        crate::config::rm_old_config().unwrap();
                        *show_old_config_popup = false;
                    }
                    if ui.button("No").clicked() {
                        *show_old_config_popup = false;
                    }
                });
        }

        custom_window_frame(ctx, _frame, "OpenXG", |ui| {
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

            let mut shown_ua = config.user_agent.clone();

            {
                let lock = crate::update_thread::SHARED_UPDATE_THREAD_STATE.lock().unwrap();

                if lock.user_agent != config.user_agent {
                    shown_ua = lock.user_agent.clone();
                }
            }

            let user_agent_edit = egui::TextEdit::singleline(&mut shown_ua)
                .hint_text("User Agent");

            ui.add(user_agent_edit);

            ui.checkbox(&mut config.set_ua_most_common, "Dynamic Stealth: Update UA to most common");

            if ui.button("Reset UA to default").clicked() {
                config.user_agent = "OpenXGAuthenticator GUI ".to_string() + libopenxg::DEFAULT_UA_SUFFIX;
                config.set_ua_most_common = false;
                config.calc_current_ua = ua::get_current_ua(config);

                {
                    let mut lock = crate::update_thread::SHARED_UPDATE_THREAD_STATE.lock().unwrap();
                    lock.user_agent = config.user_agent.clone();
                }
            }

            ui.label("Status: ".to_string() + &UA_STATUS.lock().unwrap().to_string());

            ui.separator();

            ui.label("Keep alive interval (seconds):");
            ui.add(egui::Slider::new(&mut config.keepalive_delay, 10..=120));

            ui.label("Retry interval on fail (seconds):");
            ui.add(egui::Slider::new(&mut config.retry_delay, 1..=30));

            ui.separator();

            ui.label("Do automatic updates?");
            ui.checkbox(&mut config.auto_update, "Automatic updates");

            ui.label("Status: ".to_string() + &AUTO_UPDATE_STATUS.lock().unwrap().to_string());

            if ui.button("Save").clicked() {
                // Create thread to save config
                config.calc_current_ua = ua::get_current_ua(config);

                crate::config::save_config(config).unwrap();
                update_thread::SHARED_UPDATE_THREAD_STATE.lock().unwrap().clone_from(config);
            }

            // TODO option to start at startup

            egui::warn_if_debug_build(ui);
        });
    }
}

fn custom_window_frame (
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui)
) {
    use egui::*;

    let panel_frame = egui::Frame {
        fill: ctx.style().visuals.window_fill(),
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(),
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, frame, title_bar_rect, title);

        // Add the contents:
        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}

fn title_bar_ui(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    title_bar_rect: eframe::epaint::Rect,
    title: &str,
) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        frame.set_maximized(!frame.info().window_info.maximized);
    } else if title_bar_response.is_pointer_button_down_on() {
        frame.drag_window();
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui, frame);
        });
    });
}

/// Show some close/maximize/minimize buttons for the native window.
fn close_maximize_minimize(ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    use egui::{Button, RichText};

    let button_height = 12.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        frame.close();
    }

    if frame.info().window_info.maximized {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Restore window");
        if maximized_response.clicked() {
            frame.set_maximized(false);
        }
    } else {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Maximize window");
        if maximized_response.clicked() {
            frame.set_maximized(true);
        }
    }

    let minimized_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        frame.set_minimized(true);
    }
}