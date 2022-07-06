use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct SteamApp {
    appid: i64,
    name: String,
}

use serde_json::Value;
use crate::util;

pub fn make_api_call() {
    let slash_separator = "/";
    let parameters_start = "?";
    let parameter_equals = "=";
    let key_parameter = "key";

    let steam_api_url = "https://api.steampowered.com";

    println!("Interface: ISteamApps");
    let mut interface = "ISteamApps";

    println!("Method: GetAppList");
    let mut method = "GetAppList";
    let mut version = "v2";

    let steam_web_api_key = util::getSteamWebAPIKey();

    let url = [steam_api_url, slash_separator, interface, slash_separator, method, slash_separator, version, parameters_start, key_parameter, parameter_equals, &steam_web_api_key].join("");
    println!("Request URL {}", url);


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