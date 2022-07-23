use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use serde_json::Value;
use crate::util::get_cache_dir_path;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SteamAppDetails {
    pub(crate) app_id: i64,
    pub(crate) detailed_description: String,
    pub(crate) reviews: String,
    pub(crate) header_image: String,
    pub(crate) website: String,
}

pub fn get(app_id: i64) -> SteamAppDetails {
    println!("appdetails for {}", app_id);
    let api_response = make_api_call(app_id);
    parse_api_call_result(api_response, app_id)
}

pub fn make_api_call(app_id: i64) -> String {
    let url = get_api_url(app_id);

    let response = minreq::get(url).send();
    let raw_response : Vec<u8> = response.unwrap().into_bytes();
    let response_string = String::from_utf8(raw_response).unwrap();

    let filepath = get_resource_filepath(app_id);

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

pub fn get_api_url(appId: i64) -> String {
    let api_url = format!("https://store.steampowered.com/api/appdetails?appids={}&lang=en", appId);
    api_url
}

pub fn get_resource_filepath(app_id: i64) -> String {
    let  interface = "steampowered";
    let  method = "appdetails";

    let resource = [interface, method, app_id.to_string().as_str(), get_json_filetype().as_str()].join("-");

    let filepath = [get_cache_dir_path(), "/".to_string(), resource].join("");

    filepath
}

pub fn get_json_filetype() -> String {
    "json".to_string()
}

pub fn parse_api_call_result(response_string: String, app_id: i64) -> SteamAppDetails {
    let mut json: Value = serde_json::from_str(&response_string).unwrap();

    let mut app_details_wrapped = json[app_id.to_string()].take();

    let mut app_details : Value = app_details_wrapped["data"].take();

    let boxed_reviews = app_details["reviews"].take();
    let boxed_detailed_description = app_details["detailed_description"].take();
    let boxed_header_image = app_details["header_image"].take();
    let boxed_website = app_details["website"].take();


    let mut steam_app_details = SteamAppDetails {
        app_id: app_id,
        detailed_description: "".to_string(),
        reviews: "".to_string(),
        header_image: "".to_string(),
        website: "".to_string(),
    };

    if boxed_website.as_str().is_some() {
        steam_app_details.website = boxed_website.as_str().unwrap().to_string();
    }

    if boxed_header_image.as_str().is_some() {
        steam_app_details.header_image = boxed_header_image.as_str().unwrap().to_string();
    }

    if boxed_detailed_description.as_str().is_some() {
        steam_app_details.detailed_description = boxed_detailed_description.as_str().unwrap().to_string();
    }

    if boxed_reviews.as_str().is_some() {
        steam_app_details.reviews = boxed_reviews.as_str().unwrap().to_string();
    }

    println!("steam_app_details: {}", steam_app_details.detailed_description);

    steam_app_details
}