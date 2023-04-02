use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Config {
    // the uri root of the app, e.g. if github pages, the repo name
    pub uri_root: String,
    // the media url
    media_url_inner: String,

    pub number_of_list_lines: usize
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
                number_of_list_lines: 5,
            }
        });
    } else {
        pub const CONFIG: Lazy<Config> = Lazy::new(|| {
            let uri_root = APP_CONFIG.uri_root.clone();
            Config {
                media_url_inner: if !uri_root.is_empty() {
                    format!("/{}/media", uri_root)
                } else {
                    "/media".to_string()
                },
                uri_root,
                number_of_list_lines: 5,
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

