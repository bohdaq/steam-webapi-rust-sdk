use std::fs;
use std::fs::{File, read_to_string};
use std::path::Path;
use std::io::Write;
use serde_json::Value;
use serde::Deserialize;
use crate::util::get_root_dir_path;

#[derive(Deserialize, Debug)]
pub struct SteamAppDetails {
    pub(crate) app_id: i64,
    pub(crate) name: String,
    pub(crate) detailed_description: String,
    pub(crate) reviews: String,
    pub(crate) header_image: String,
    pub(crate) website: String,
}

pub fn get(app_id: i64) -> Result<SteamAppDetails, String> {
    println!("appdetails for {}", app_id);
    let api_response_boxed = make_api_call(app_id);
    if api_response_boxed.is_err() {
        return Err(api_response_boxed.err().unwrap().to_string());
    } else {
        parse_api_call_result(api_response_boxed.unwrap(), app_id)
    }
}

pub fn get_cached(app_id: i64) -> Result<SteamAppDetails, String> {
    let filepath = get_resource_filepath(app_id);
    println!("{}", &filepath);

    let boxed_read = read_to_string(filepath);
    let is_readable = boxed_read.is_ok();
    if is_readable {
        let cached_api_response = boxed_read.unwrap();
        parse_api_call_result(cached_api_response, app_id)
    } else {
        println!("unable to read from cache, invoking api");
        get(app_id)
    }

}

pub fn make_api_call(app_id: i64) -> Result<String, String> {
    let url = get_api_url(app_id);

    let boxed_response = minreq::get(url).send();
    if boxed_response.is_err() {
        return Err("Operation timed out (API call)".to_string());
    }

    let raw_response : Vec<u8> = boxed_response.unwrap().into_bytes();
    println!("raw_response length {}", raw_response.len());

    let response_string_boxed = String::from_utf8(raw_response);
    if response_string_boxed.is_err() {
        let error_message = response_string_boxed.err().unwrap().to_string();
        println!("{}", &error_message);
        if error_message == "invalid utf-8 sequence of 1 bytes from index 1" {
            return Err("no response from API".to_string());
        }
        return Err("invalid utf-8 sequence".to_string());
    }
    let response_string: String = response_string_boxed.unwrap();
    println!("make_api_call response_string {}", response_string.len());

    Ok(response_string)
}

pub fn get_api_url(appId: i64) -> String {
    let api_url = format!("https://store.steampowered.com/api/appdetails?appids={}&lang=en", appId);
    api_url
}

pub fn get_resource_filepath(app_id: i64) -> String {
    let cache_dir = get_cache_dir_path(app_id);
    let filepath = [
        cache_dir,
        app_id.to_string(),
        ".".to_string(),
        get_json_filetype(),
    ].join("");
    filepath
}

pub fn get_json_filetype() -> String {
    "json".to_string()
}

pub fn parse_api_call_result(response_string: String, app_id: i64) -> Result<SteamAppDetails, String> {
    let mut steam_app_details = SteamAppDetails {
        app_id: app_id,
        name: "".to_string(),
        detailed_description: "".to_string(),
        reviews: "".to_string(),
        header_image: "".to_string(),
        website: "".to_string(),
    };

    if response_string.len() > 0 {
        let boxed_initial_parse = serde_json::from_str(&response_string);
        if boxed_initial_parse.is_err() {
            println!("{}", &response_string);
            return Err(boxed_initial_parse.err().unwrap().to_string());
        }
        let mut json: Value = boxed_initial_parse.unwrap();

        let mut app_details_wrapped = json[app_id.to_string()].take();

        let mut is_success = app_details_wrapped["success".to_string()].take();
        if is_success.take().as_bool().unwrap() == false {
            return Err("steampowered api returned failed response".to_string());
        }

        let mut app_details : Value = app_details_wrapped["data"].take();

        let boxed_reviews = app_details["reviews"].take();
        let boxed_detailed_description = app_details["detailed_description"].take();
        let boxed_header_image = app_details["header_image"].take();
        let boxed_website = app_details["website"].take();
        let boxed_name = app_details["name"].take();


        if boxed_name.as_str().is_some() {
            steam_app_details.name = boxed_name.as_str().unwrap().to_string();
        }

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

        // println!("steam_app_details: {}", steam_app_details.detailed_description);
    }

    let filepath = get_resource_filepath(app_id);

    let mut file: File;
    let directory_exists = Path::new(get_cache_dir_path(app_id).as_str()).is_dir();
    if !directory_exists {
        fs::create_dir_all(get_cache_dir_path(app_id)).unwrap();
        file = File::create(filepath).unwrap();
    } else {
        file = File::create(filepath).unwrap();
    }

    file.write_all(response_string.as_ref()).unwrap();

    Ok(steam_app_details)
}

pub fn get_cache_dir_path(app_id: i64) -> String {
    let root_path  = get_root_dir_path();
    let  interface = "steampowered";
    let  method = "appdetails";
    let number_of_entries_per_bucket = 10000;
    let bucket = app_id / number_of_entries_per_bucket;

    [
        root_path,
        "/".to_string(),
        "cache".to_string(),
        "/".to_string(),
        interface.to_string(),
        "/".to_string(),
        method.to_string(),
        "/".to_string(),
        bucket.to_string(),
        "/".to_string(),
        app_id.to_string(),
        "/".to_string()
    ].join("")
}