use std::collections::HashMap;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_steam_web_api_key() -> String {
    let boxed_steam_web_api_key = env::var("STEAM_WEBAPI_KEY");
    if boxed_steam_web_api_key.is_err() {
        println!("To use this SDK please specify STEAM_WEBAPI_KEY environment variable");
        return "".to_string();
    }
    let steam_web_api_key = boxed_steam_web_api_key.unwrap();
    let _key = ["STEAM_WEB_API_KEY is ", &steam_web_api_key].join("");

    return steam_web_api_key;
}

pub fn build_api_url(interface: &str, method: &str, version: &str, _parameters: HashMap<String, String>) -> String {
    let slash_separator = "/";
    let parameters_start = "?";
    let parameter_equals = "=";
    let key_parameter = "key";

    let steam_api_url = "https://api.steampowered.com";

    let steam_web_api_key = get_steam_web_api_key();

    let url = [steam_api_url, slash_separator, interface, slash_separator, method, slash_separator, version, parameters_start, key_parameter, parameter_equals, &steam_web_api_key].join("");

    return url
}

pub fn get_cache_dir_path() -> String {
    ["cache".to_string()].join("/")
}

pub fn as_unix_timestamp(system_time: SystemTime) -> u64 {
    let since_the_epoch = system_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let unix_timestamp = since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    unix_timestamp
}