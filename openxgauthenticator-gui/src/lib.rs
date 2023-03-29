#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod config;
mod update_thread;
mod ua;
mod self_update;
pub use app::OpenXGApp;