use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SteamApp {
    appid: i64,
    name: String,
}

use serde_json::Value;
use crate::util;

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
    println!("{}", response_string);

    response_string
}

pub fn parse_api_call_result(response_string: String) -> Vec<SteamApp> {
    let mut json: Value = serde_json::from_str(&response_string).unwrap();

    let mut applist = json["applist"].take();
    println!("{}, applist", applist);

    let mut apps : Value = applist["apps"].take();
    println!("{}, apps", apps);

    let list : Vec<SteamApp> = serde_json::from_value(apps).unwrap();
    println!("{}, apps number", list.len());


    list
}