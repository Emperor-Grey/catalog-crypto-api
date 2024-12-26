use std::env;

pub fn get_midgard_api_url() -> String {
    env::var("MIDGARD_API_URL").unwrap_or_else(|_| "http://rick_roll.com".to_string())
}
