use std::env;

pub struct Config {
    pub api_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_url: env::var("API_URL").unwrap_or_else(|_| "https://api.example.com".to_owned()),
        }
    }
}
