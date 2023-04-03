#![warn(clippy::all, rust_2018_idioms)]

pub const EMBEDDED_IMG_DATA: &[u8; 59897] = include_bytes!("../../icon.png");

mod app;
mod config;
mod update_thread;
mod ua;
mod self_update;
pub use app::OpenXGApp;