//! # Steam Web API Rust SDK
//!
//! `steam-webapi-rust-sdk` is a set of utility functions to access Steam Web API.
//!
//! In order to use this library make sure to set STEAM_WEB_API_KEY system environment variable.
//!
//! The library itself tries to minimize number of networks calls through the caching relevant
//! responses to the 'steam-webapi-cache' folder.
//!
//! There is already prebuilt cache for all steam apps, in order to use it,
//! simply clone [steam-webapi-cache](https://github.com/bohdaq/steam-webapi-cache)
//! into the root folder of your project.

use crate::isteam_apps::get_app_list::SteamApp;
use crate::store_steampowered_com::appdetails::SteamAppDetails;

pub mod util;
pub mod isteam_apps;
pub mod store_steampowered_com;

#[cfg(test)]
mod tests;

/// Retrieves details for the given app id from the local cache. It may return an error
/// if requested resource is absent, malformed or not readable from local cache.
///
/// # Examples
///
/// ```
/// let app_id = 570;
/// let boxed_result = steam_webapi_rust_sdk::get_cached_app_details(app_id);
/// if boxed_result.is_ok() {
///     let app_details = boxed_result.unwrap();
///     println!("result is ok for {} app id {}", app_details.name, app_details.app_id);
///
/// } else {
///     let error_message = boxed_result.err().unwrap();
///     println!("{} {}", error_message, app_id);
///
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
/// let boxed_result = steam_webapi_rust_sdk::get_app_details(app_id);
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
///     let exceeded_api_calls_limit = (!is_steam_unsuccessful_response && !is_invalid_utf8_sequence) || no_response_from_api;
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
/// let steam_app_list = steam_webapi_rust_sdk::get_app_list().unwrap();
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
/// let steam_app_list = steam_webapi_rust_sdk::get_cached_app_list().unwrap();
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
