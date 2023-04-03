#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use openxgauthenticator_gui::EMBEDDED_IMG_DATA;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    //let img_embedded = include_bytes!("../../icon.png");
    let icon = image::load_from_memory(EMBEDDED_IMG_DATA).expect("Failed to open icon").to_rgba8();

    let (w, h) = icon.dimensions();

    let mut native_options = eframe::NativeOptions {
        //resizable: true,
        initial_window_size: Some(egui::Vec2 {x: 500.0, y: 600.0}),
        min_window_size: Some(egui::Vec2 {x: 500.0, y: 600.0}),
        icon_data: Some(eframe::IconData {
            rgba: icon.into_raw(),
            width: w,
            height: h,
        }),
        decorated: false,
        transparent: true,
        ..Default::default()
    };

    eframe::run_native(
        "OpenXG",
        native_options,
        Box::new(|cc| Box::new(openxgauthenticator_gui::OpenXGApp::new(cc))),
    )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
