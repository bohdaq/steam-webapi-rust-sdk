use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SteamApp {
    pub(crate) appid: i64,
    pub(crate) name: String,
}

use serde_json::Value;
use crate::{isteam_apps, util};
use std::fs;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;
use crate::util::get_cache_dir_path;

pub fn get() -> Vec<SteamApp> {
    println!("getting app list");
    let api_response = make_api_call();
    parse_api_call_result(api_response)
}

pub fn get_cached() -> Vec<SteamApp> {
    let filepath = get_resource_filepath();

    let boxed_read = read_to_string(filepath);
    let is_readable = boxed_read.is_ok();
    if is_readable {
        let cached_api_response = boxed_read.unwrap();
        parse_api_call_result(cached_api_response)
    } else {
        println!("{} {} {} unable to read from cache, invoking api", isteam_apps::get_interface(), get_method_name(), get_version());
        get()
    }

}

pub fn get_method_name() -> String {
    "GetAppList".to_string()
}

pub fn get_version() -> String {
    "v2".to_string()
}

pub fn get_json_filetype() -> String {
    "json".to_string()
}

pub fn get_resource_filepath() -> String {
    let  interface = isteam_apps::get_interface();
    let  method = get_method_name();
    let  version = get_version();

    let resource = [interface, method, version, get_json_filetype()].join("-");

    let filepath = [get_cache_dir_path(), "/".to_string(), resource].join("");

    filepath
}

pub fn get_api_url() -> String {
    let  interface = isteam_apps::get_interface();
    let  method = get_method_name();
    let  version = get_version();
    let parameters : HashMap<String, String> = HashMap::new();

    let api_url = util::build_api_url(interface.as_str(), method.as_str(), version.as_str(), parameters);

    api_url
}

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

pub fn parse_api_call_result(response_string: String) -> Vec<SteamApp> {
    let mut json: Value = serde_json::from_str(&response_string).unwrap();

    let mut applist = json["applist"].take();

    let apps : Value = applist["apps"].take();

    let list : Vec<SteamApp> = serde_json::from_value(apps).unwrap();

    let filtered_list = list
                            .into_iter()
                            .filter(|steam_app| steam_app.name != "")
                            .collect();

    filtered_list
}