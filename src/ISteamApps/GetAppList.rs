use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SteamApp {
    pub(crate) appid: i64,
    pub(crate) name: String,
}

use serde_json::Value;
use crate::util;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn get() -> Vec<SteamApp> {
    let api_response = make_api_call();
    parse_api_call_result(api_response)
}

pub fn make_api_call() -> String {
    println!("Interface: ISteamApps");
    let mut interface = "ISteamApps";

    println!("Method: GetAppList");
    let mut method = "GetAppList";
    let mut version = "v2";

    let mut parameters : HashMap<String, String> = HashMap::new();

    let url = util::build_api_url(interface, method, version, parameters);

    let response = minreq::get(url).send();
    let raw_response : Vec<u8> = response.unwrap().into_bytes();
    let response_string = String::from_utf8(raw_response).unwrap();

    let path = "cache";
    let resource = [interface, method, version, "json"].join("-");

    let mut file: File;
    let filepath = [path, "/", &resource].join("");


    let directory_exists = Path::new(path).is_dir();
    if !directory_exists {
        fs::create_dir(path).unwrap();
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