use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct SteamApp {
    pub appid: i64,
    pub name: String,
}

use serde_json::Value;
use crate::{isteam_apps, util};
use std::fs;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;
use crate::util::{get_cache_dir_path, get_json_filetype};


/// Retrieves apps available on the Steam store.
///
/// # Examples
///
/// ```
/// let app_list = steam_webapi_rust_sdk::isteam_apps::get_app_list::get().unwrap();
///
/// assert!(app_list.len() > 0);
/// ```
pub fn get() -> Result<Vec<SteamApp>, String> {
    let api_response = make_api_call();
    parse_api_call_result(api_response)
}

/// Will get cached response if present, otherwise retrieves apps
/// available on the Steam store
/// and puts it to local cache folder.
///
/// # Examples
///
/// ```
/// let app_list = steam_webapi_rust_sdk::isteam_apps::get_app_list::get_cached().unwrap();
///
/// assert!(app_list.len() > 0);
/// let steam_app = app_list.get(0).unwrap();
/// assert!(steam_app.appid > 0);
///
/// assert!(steam_app.name.len() > 0);
/// ```
pub fn get_cached() -> Result<Vec<SteamApp>, String> {
    let filepath = get_resource_filepath();

    let boxed_read = read_to_string(filepath);
    let is_readable = boxed_read.is_ok();
    if is_readable {
        let cached_api_response = boxed_read.unwrap();
        parse_api_call_result(cached_api_response)
    } else {
        get()
    }

}

/// Returns method name invoked on Steam API.
///
/// # Examples
///
/// ```
/// let method_name = steam_webapi_rust_sdk::isteam_apps::get_app_list::get_method_name();
///
/// assert!(method_name == "GetAppList".to_string());
/// ```
pub fn get_method_name() -> String {
    "GetAppList".to_string()
}


/// Returns version of the method invoked on Steam API.
///
/// # Examples
///
/// ```
/// let version = steam_webapi_rust_sdk::isteam_apps::get_app_list::get_version();
///
/// assert!(version == "v2".to_string());
/// ```
pub fn get_version() -> String {
    "v2".to_string()
}


/// Returns path to the cached resource invoked on Steam API.
/// The resource on the given filepath can be absent
/// if the get method was not invoked previously.
///
///
/// # Examples
///
/// ```
/// let version = steam_webapi_rust_sdk::isteam_apps::get_app_list::get_resource_filepath();
///
/// assert!(version == "steam-webapi-cache/ISteamApps-GetAppList-v2.json".to_string());
/// ```
pub fn get_resource_filepath() -> String {
    let  interface = isteam_apps::get_interface();
    let  method = get_method_name();
    let  version = get_version();

    let resource = [interface, "-".to_string(), method, "-".to_string(), version, ".".to_string(), get_json_filetype()].join("");

    let filepath = [get_cache_dir_path(), "/".to_string(), resource].join("");

    filepath
}

/// Returns API url invoked on Steam API.
///
///
/// # Examples
///
/// ```
/// let api_url = steam_webapi_rust_sdk::isteam_apps::get_app_list::get_api_url();
///
/// let steam_web_api_key = steam_webapi_rust_sdk::util::get_steam_web_api_key();
/// let expected_api_url = ["https://api.steampowered.com/ISteamApps/GetAppList/v2?key=".to_string(), steam_web_api_key].join("");
///
/// assert_eq!(api_url, expected_api_url.to_string());
/// ```
pub fn get_api_url() -> String {
    let  interface = isteam_apps::get_interface();
    let  method = get_method_name();
    let  version = get_version();
    let parameters : HashMap<String, String> = HashMap::new();

    let api_url = util::build_api_url(interface.as_str(), method.as_str(), version.as_str(), parameters);

    api_url
}

/// Makes API call and returns response body.
///
///
/// # Examples
///
/// ```
/// let response = steam_webapi_rust_sdk::isteam_apps::get_app_list::make_api_call();
/// assert!(response.len()>0);
/// ```
pub fn make_api_call() -> String {
    let url = get_api_url();

    let response = minreq::get(url).send();
    let raw_response : Vec<u8> = response.unwrap().into_bytes();
    let response_string = String::from_utf8(raw_response).unwrap();

    let filepath = get_resource_filepath();

    let mut file: File;
    let directory_exists = Path::new(get_cache_dir_path().as_str()).is_dir();
    if !directory_exists {
        fs::create_dir_all(get_cache_dir_path()).unwrap();
        file = File::create(filepath).unwrap();
    } else {
        file = File::create(filepath).unwrap();
    }

    file.write_all(response_string.as_ref()).unwrap();

    response_string
}

/// Parses API call response body.
///
///
/// # Examples
///
/// ```
/// let response = steam_webapi_rust_sdk::isteam_apps::get_app_list::make_api_call();
/// let boxed_steam_app_list = steam_webapi_rust_sdk::isteam_apps::get_app_list::parse_api_call_result(response);
/// assert!(boxed_steam_app_list.is_ok());
///
/// let steam_app_list = boxed_steam_app_list.unwrap();
/// assert!(steam_app_list.len() > 0);
///
/// let steam_app = steam_app_list.get(0).unwrap();
/// assert!(steam_app.appid > 0);
/// assert!(steam_app.name.len() > 0);
/// ```
pub fn parse_api_call_result(response_string: String) -> Result<Vec<SteamApp>, String> {
    let mut json: Value = serde_json::from_str(&response_string).unwrap();

    let mut applist = json["applist"].take();

    let apps : Value = applist["apps"].take();

    let list : Vec<SteamApp> = serde_json::from_value(apps).unwrap();

    let filtered_list = list
                            .into_iter()
                            .filter(|steam_app| steam_app.name != "")
                            .collect();

    Ok(filtered_list)
}