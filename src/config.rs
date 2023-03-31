use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Config {
    // the uri root of the app, e.g. if github pages, the repo name
    pub uri_root: String,
    // the media url
    media_url_inner: String,
}

impl Config {
    pub fn media_url(&self, path: &str) -> String {
        format!("{}/{}", self.media_url_inner, path)
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "dev")] {
        pub const CONFIG: Lazy<Config> = Lazy::new(|| {
            Config {
                // in dev mode it's always served from top-level
                uri_root: "".to_string(),
                media_url_inner: "http://127.0.0.1:9000".to_string(),
            }
        });
    } else {
        pub const CONFIG: Lazy<Config> = Lazy::new(|| {
            Config {
                uri_root: APP_CONFIG.uri_root.clone(), 
                media_url_inner: format!("{}/media", APP_CONFIG.uri_root),
            }
        });
    }
}

// this is just used for loading hardcoded values, and is private to this module
// the runtime config is via the exported CONFIG
const APP_CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    serde_json::from_str(APP_CONFIG_JSON_STR).unwrap()
});

const APP_CONFIG_JSON_STR: &str = include_str!("../app.config.json");
#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    uri_root: String,
}

