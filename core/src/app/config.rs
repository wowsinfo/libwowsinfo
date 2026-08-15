//! Shell-supplied configuration.

use facet::Facet;
use serde::{Deserialize, Serialize};

use crate::data::{self, Server};

/// Startup configuration supplied by the shell.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Facet, PartialEq)]
pub struct Config {
    pub server: Server,
    #[serde(default = "default_language")]
    pub language: String,
    /// Optional override; falls back to the key embedded by `build.rs`.
    #[serde(default)]
    pub api_key: String,
}

fn default_language() -> String {
    data::DEFAULT_API_LANGUAGE.to_string()
}
