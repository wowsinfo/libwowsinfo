#![allow(clippy::unsafe_derive_deserialize)]

mod app;

pub mod arena;
pub mod api;
pub mod charts;
pub mod combat;
pub mod data;
pub mod downloader;
pub mod models;
pub mod rating;
pub mod util;
pub mod wiki;
pub mod warship;

pub mod ffi;

pub use app::*;
pub use crux_core::Core;
pub use data::Server;
pub use models::*;

/// Wargaming API key injected by `build.rs` from `keys.toml` or `WOWSINFO_APP_KEY`.
pub const APP_KEY: &str = env!("WOWSINFO_APP_KEY");
