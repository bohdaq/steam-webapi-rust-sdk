use std::collections::HashMap;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;
use url_build_parse::{build_url, UrlAuthority, UrlComponents};

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
    ["steam-webapi-cache".to_string()].join("/")
}

pub fn as_unix_timestamp(system_time: SystemTime) -> u64 {
    let since_the_epoch = system_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let unix_timestamp = since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    unix_timestamp
}

pub fn get_json_filetype() -> String {
    "json".to_string()
}

/// Builds a `https://api.steampowered.com/<interface>/<method>/<version>?<params>` URL.
pub fn build_steam_api_url(interface: &str, method: &str, version: &str, params_map: HashMap<String, String>) -> String {
    let path = ["/".to_string(), interface.to_string(), "/".to_string(), method.to_string(), "/".to_string(), version.to_string()].join("");

    let url_builder = UrlComponents {
        scheme: "https".to_string(),
        authority: Some(UrlAuthority {
            user_info: None,
            host: "api.steampowered.com".to_string(),
            port: None
        }),
        path,
        query: Some(params_map),
        fragment: None
    };

    build_url(url_builder).unwrap()
}

/// Reads an unsigned integer field from a JSON object, defaulting to 0 if absent or the wrong type.
pub fn json_u64(value: &Value, key: &str) -> u64 {
    value.get(key).and_then(Value::as_u64).unwrap_or(0)
}

/// Reads a signed integer field from a JSON object, defaulting to 0 if absent or the wrong type.
pub fn json_i64(value: &Value, key: &str) -> i64 {
    value.get(key).and_then(Value::as_i64).unwrap_or(0)
}

/// Reads a floating point field from a JSON object, defaulting to 0.0 if absent or the wrong type.
pub fn json_f64(value: &Value, key: &str) -> f64 {
    value.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

/// Reads a string field from a JSON object, defaulting to an empty string if absent or the wrong type.
pub fn json_str(value: &Value, key: &str) -> String {
    value.get(key).and_then(Value::as_str).unwrap_or("").to_string()
}

/// Reads a boolean field from a JSON object, defaulting to false if absent or the wrong type.
pub fn json_bool(value: &Value, key: &str) -> bool {
    value.get(key).and_then(Value::as_bool).unwrap_or(false)
}