//! # Steam Web API Rust SDK
//!
//! `steam-webapi-rust-sdk` is a set of utility functions to access Steam Web API.
//!
//! In order to use this library make sure to set STEAM_WEB_API_KEY system environment variable.

// curl https://api.steampowered.com/IDOTA2Match_570/GetMatchHistory/V001/\?key\=1F2709FC907F0DEE1D1EB4787E06B695\&account_id\=1187853121
// curl https://api.steampowered.com/IDOTA2Match_570/GetMatchDetails/V001/\?match_id\=6644665007\&key\=1F2709FC907F0DEE1D1EB4787E06B695
// curl https://api.steampowered.com/IEconDOTA2_205790/GetHeroes/V001/\?key\=1F2709FC907F0DEE1D1EB4787E06B695
// curl https://api.steampowered.com/IEconDOTA2_205790/GetGameItems/V001/\?key\=1F2709FC907F0DEE1D1EB4787E06B695
// curl https://api.steampowered.com/IEconDOTA2_205790/GetGameItems/V001/\?key\=1F2709FC907F0DEE1D1EB4787E06B695
// curl https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/\?key\=1F2709FC907F0DEE1D1EB4787E06B695\&steamids\=76561199148118849,76561197998367327
// http://whatsmysteamid.azurewebsites.net/
// https://store.steampowered.com/api/appdetails?appids=730&lang=ua
// https://cdn.akamai.steamstatic.com/steam/apps/730/capsule_231x87.jpg
// https://cdn.dota2.com/apps/dota2/images/items/super_blink_lg.png
// https://cdn.dota2.com/apps/dota2/images/heroes/dark_willow_full.png


use std::{thread, time};
use std::fs::{File, OpenOptions, read_to_string};
use std::io::Write;
use std::path::Path;
use crate::isteam_apps::get_app_list::SteamApp;
use crate::store_steampowered_com::appdetails::SteamAppDetails;
use crate::util::get_cache_dir_path;

pub mod util;
pub mod isteam_apps;
pub mod store_steampowered_com;

#[cfg(test)]
mod main_test;

fn main() {
    println!("Steam Web API Rust SDK");

    let app_list = get_app_list().unwrap();
    let mut iteration_number = 1;
    let app_list_size = app_list.len();

    let mut processed_app_id_list: Vec<i64> = vec![];

    let already_processed_app_id_list_path = [get_cache_dir_path(), "/".to_string(), "processed_app_id_list.json".to_string()].join("");
    let file_exists = Path::new(already_processed_app_id_list_path.as_str()).is_file();
    if file_exists {
        let serialized_string = read_to_string(&already_processed_app_id_list_path).unwrap();
        if serialized_string.len() > 0 {
            processed_app_id_list = serde_json::from_str(serialized_string.as_str()).unwrap();
        }
    } else {
        File::create(&already_processed_app_id_list_path).unwrap();
    }

    println!("Filtering already processed app details. This may take a while...");
    let mut iteration = 0;
    let filtered_list: Vec<SteamApp> = app_list
        .into_iter()
        .filter(|steam_app| {
            iteration = iteration + 1;
            println!("Iteration {} of {}", iteration, app_list_size);
            !processed_app_id_list.contains(&steam_app.appid)

        })
        .collect();

    let filtered_list_len = filtered_list.len();


    for app in filtered_list {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&already_processed_app_id_list_path)
            .unwrap();
        let calculated_percentage = (100_f32 * iteration_number as f32) / filtered_list_len as f32;


        println!("\n\n Iteration number: {} \n App List size:    {}  {}%  After filtering: {}", iteration_number, app_list_size, calculated_percentage, filtered_list_len);
        retrieve_detailed_app_info(app.appid);
        iteration_number = iteration_number + 1;
        processed_app_id_list.push(app.appid);

        let serialized_list = serde_json::to_string(&processed_app_id_list).unwrap();
        file.write_all(serialized_list.as_ref()).unwrap();
    }
}

fn retrieve_detailed_app_info(app_id: i64) {
    let boxed_result = get_cached_app_details(app_id);
    if boxed_result.is_ok() {
        let app_details = boxed_result.unwrap();
        println!("result is ok for {} app id {}", app_details.name, app_details.app_id);

    } else {
        let error_message = boxed_result.err().unwrap();
        println!("{} {}", error_message, app_id);

        let is_not_steam_unsuccessful_response = error_message != "steampowered api returned failed response";
        let is_not_invalid_utf8_sequence = error_message != "invalid utf-8 sequence";
        let no_response_from_api = error_message == "no response from API";

        if (is_not_steam_unsuccessful_response && is_not_invalid_utf8_sequence) || no_response_from_api {
            println!("result is not ok for app id {}, retry in 1 min ", app_id);

            let one_minute = time::Duration::from_secs(60);
            thread::sleep(one_minute);

            retrieve_detailed_app_info(app_id);
        }
    }
}




/// Retrieves details for the given app id. First tries to get it from the local cache,
/// if not present will make an API call to Steam and cache response. It may return an error
/// if API responded with error response. As an example it may be exceeding the limit of calls
/// from one IP address or if the response contains not valid UTF-8 characters.
/// Usually Steam API allows 200 requests from single IP address within 5 minutes range.
///
/// # Examples
///
/// ```
/// let app_id = 570;
/// let boxed_result = get_cached_app_details(app_id);
/// if boxed_result.is_ok() {
///     let app_details = boxed_result.unwrap();
///     println!("result is ok for {} app id {}", app_details.name, app_details.app_id);
///
/// } else {
///     let error_message = boxed_result.err().unwrap();
///     println!("{} {}", error_message, app_id);
///
///     let is_steam_unsuccessful_response = error_message == "steampowered api returned failed response";
///     let is_invalid_utf8_sequence = error_message == "invalid utf-8 sequence";
///     let no_response_from_api = error_message == "no response from API";
///     let exceeded_api_calls_limit = (!is_steam_unsuccessful_response && !is_invalid_utf8_sequence) || no_response_from_api
///
///     // you can do a retry or continue execution...
/// };
/// ```
pub fn get_cached_app_details(app_id: i64) -> Result<SteamAppDetails, String> {
    let boxed_result = store_steampowered_com::appdetails::get_cached(app_id);
    boxed_result
}

/// Retrieves details for the given app id. It will make an API call to Steam and cache response.
/// It may return an error if API responded with error response. As an example it may be exceeding
/// the limit of calls from one IP address or if the response contains not valid UTF-8 characters.
/// Usually Steam API allows 200 requests from single IP address within 5 minutes range.
///
/// # Examples
///
/// ```
/// let app_id = 570;
/// let boxed_result = get_app_details(app_id);
/// if boxed_result.is_ok() {
///     let app_details = boxed_result.unwrap();
///     println!("result is ok for {} app id {}", app_details.name, app_details.app_id);
///
/// } else {
///     let error_message = boxed_result.err().unwrap();
///     println!("{} {}", error_message, app_id);
///
///     let is_steam_unsuccessful_response = error_message == "steampowered api returned failed response";
///     let is_invalid_utf8_sequence = error_message == "invalid utf-8 sequence";
///     let no_response_from_api = error_message == "no response from API";
///     let exceeded_api_calls_limit = (!is_steam_unsuccessful_response && !is_invalid_utf8_sequence) || no_response_from_api
///
///     // you can do a retry or continue execution...
/// };
/// ```
pub fn get_app_details(app_id: i64) -> Result<SteamAppDetails, String> {
    let boxed_result = store_steampowered_com::appdetails::get(app_id);
    boxed_result
}

/// Retrieves list of apps available on Steam. Each item consists of 2 fields: appid and name
///
/// # Examples
///
/// ```
/// let steam_app_list = get_app_list().unwrap();
///
/// assert!(steam_app_list.len()>0);
/// let steam_app = steam_app_list.get(0).unwrap();
/// assert!(steam_app.appid > 0);
///
/// assert!(steam_app.name.len() > 0);
/// ```
pub fn get_app_list() -> Result<Vec<SteamApp>, String> {
    let boxed_result = isteam_apps::get_app_list::get();
    boxed_result
}


/// Retrieves list of apps available on Steam. First tries to get it from local cache.
/// Each item consists of 2 fields: appid and name
///
/// # Examples
///
/// ```
/// let steam_app_list = get_cached_app_list().unwrap();
///
/// assert!(steam_app_list.len()>0);
/// let steam_app = steam_app_list.get(0).unwrap();
/// assert!(steam_app.appid > 0);
///
/// assert!(steam_app.name.len() > 0);
/// ```
pub fn get_cached_app_list() -> Result<Vec<SteamApp>, String> {
    let boxed_result = isteam_apps::get_app_list::get_cached();
    boxed_result
}
