use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SteamApp {
    pub(crate) appid: i64,
    pub(crate) name: String,
}

use serde_json::Value;
use crate::{ISteamApps, util};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::util::get_cache_dir_path;

pub fn get() -> Vec<SteamApp> {
    let api_response = make_api_call();
    parse_api_call_result(api_response)
}

pub fn get_cached() -> Vec<SteamApp> {
    vec![]
}

fn get_method_name() -> String {
    "GetAppList".to_string()
}

fn get_version() -> String {
    "v2".to_string()
}

fn get_json_filetype() -> String {
    "json".to_string()
}

pub fn make_api_call() -> String {
    let mut interface = ISteamApps::get_interface();
    println!("Interface: {}", interface);

    let  method = get_method_name();
    println!("Method: {}", method);

    let  version = get_version();
    println!("Version: {}", version);

    let mut parameters : HashMap<String, String> = HashMap::new();

    let url = util::build_api_url(interface.as_str(), method.as_str(), version.as_str(), parameters);

    let response = minreq::get(url).send();
    let raw_response : Vec<u8> = response.unwrap().into_bytes();
    let response_string = String::from_utf8(raw_response).unwrap();

    let path = get_cache_dir_path();
    let resource = [interface, method, version, get_json_filetype()].join("-");

    let mut file: File;
    let filepath = [get_cache_dir_path(), "/".to_string(), resource].join("");


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

pub fn parse_api_call_result(response_string: String) -> Vec<SteamApp> {
    let mut json: Value = serde_json::from_str(&response_string).unwrap();

    let mut applist = json["applist"].take();

    let mut apps : Value = applist["apps"].take();

    let list : Vec<SteamApp> = serde_json::from_value(apps).unwrap();

    let filtered_list = list
                            .into_iter()
                            .filter(|steam_app| steam_app.name != "")
                            .collect();

    filtered_list
}