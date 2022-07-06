use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct SteamApp {
    appid: i64,
    name: String,
}

use serde_json::Value;
use crate::util;

pub fn make_api_call() {


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

    let mut json: Value = serde_json::from_str(&response_string).unwrap();

    let mut applist = json["applist"].take();
    println!("{}, applist", applist);

    let mut apps : Value = applist["apps"].take();
    println!("{}, apps", apps);

    let list : Vec<SteamApp> = serde_json::from_value(apps).unwrap();
    println!("{}, apps number", list.len());

    let index = list.len() - 1;
    println!("{}, app", list.get(index).unwrap().name);
}