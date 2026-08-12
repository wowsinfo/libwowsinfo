use std::{env, fs, path::PathBuf};

/// Read the Wargaming API key for this build.
///
/// Precedence: `WOWSINFO_APP_KEY` env var, then `keys.toml` at the repo root.
/// The key is exposed to the crate as `env!("WOWSINFO_APP_KEY")`; an empty
/// string means the shell should surface the `MissingKey` error at runtime.
fn main() {
    let key = if let Ok(key) = env::var("WOWSINFO_APP_KEY") {
        key
    } else {
        let manifest_dir =
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
        let keys_path = manifest_dir
            .parent()
            .expect("core is one level below repo root")
            .join("keys.toml");
        read_app_key(&keys_path)
    };

    println!("cargo:rerun-if-env-changed=WOWSINFO_APP_KEY");
    println!("cargo:rerun-if-changed={}", keys_path());
    println!("cargo:rustc-env=WOWSINFO_APP_KEY={key}");
}

fn keys_path() -> String {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .expect("core is one level below repo root")
        .join("keys.toml")
        .to_string_lossy()
        .into_owned()
}

/// Minimal TOML reader for `app_key = "..."`; keeps the build script dependency-free.
fn read_app_key(path: &std::path::Path) -> String {
    let Ok(content) = fs::read_to_string(path) else {
        return String::new();
    };
    for line in content.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("app_key") {
            if let Some(eq) = rest.find('=') {
                let value = rest[eq + 1..].trim();
                let value = value.trim_matches('"').trim();
                return value.to_string();
            }
        }
    }
    String::new()
}
