use std::collections::HashMap;
use std::env;
use std::iter::Map;

pub fn get_steam_web_api_key() -> String {
    let boxed_steam_web_api_key = env::var("STEAM_WEBAPI_KEY");
    if boxed_steam_web_api_key.is_err() {
        println!("To use this SDK please specify STEAM_WEBAPI_KEY environment variable");
        return "".to_string();
    }
    let steam_web_api_key = boxed_steam_web_api_key.unwrap();
    let _key = ["STEAM_WEB_API_KEY is ", &steam_web_api_key].join("");

    println!("{}", _key);

    return steam_web_api_key;
}

pub fn build_api_url(interface: &str, method: &str, version: &str, parameters: HashMap<String, String>) -> String {
    let slash_separator = "/";
    let parameters_start = "?";
    let parameter_equals = "=";
    let key_parameter = "key";

    let steam_api_url = "https://api.steampowered.com";

    let steam_web_api_key = get_steam_web_api_key();

    let url = [steam_api_url, slash_separator, interface, slash_separator, method, slash_separator, version, parameters_start, key_parameter, parameter_equals, &steam_web_api_key].join("");
    println!("Request URL {}", url);

    return url
}

pub fn get_root_dir_path() -> String {
    "steam-webapi-rust-sdk".to_string()
}

pub fn get_cache_dir_path() -> String {
    let root_path  = get_root_dir_path();
    [root_path, "cache".to_string()].join("/")
}