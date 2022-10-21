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
pub mod idota2match_570;

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

/// Retrieves list of matches from Dota2
///
/// # Examples
///
/// ```
/// let boxed_dota2_match_list = steam_webapi_rust_sdk::get_dota2_match_history(
///     Some(76561197960361544),
///     None,
///     None,
///     None,
///     None,
///     None,
///     None
/// );
///
/// assert!(boxed_dota2_match_list.is_ok());
/// if boxed_dota2_match_list.is_ok() {
///     let dota2_match_list = boxed_dota2_match_list.unwrap();
///     assert!(dota2_match_list.matches.len()>0);
/// }
///
/// ```
pub fn get_dota2_match_history(account_id: Option<i64>,
                               game_mode: Option<u8>,
                               skill: Option<u8>,
                               min_players: Option<u32>,
                               start_at_match_id: Option<u64>,
                               matches_requested: Option<u32>,
                               tournament_games_only: Option<bool>)
    -> Result<ResponseMatchHistory, String> {
    idota2match_570::get_dota2_match_history(
        account_id,
        game_mode,
        skill,
        min_players,
        start_at_match_id,
        matches_requested,
        tournament_games_only
    )

}

/// Converts given 32 bit Steam account id to 64 bit
///
/// # Examples
///
/// ```
/// use steam_webapi_rust_sdk::convert_32bit_account_id_to_64bit;
///
/// let _32bit_id = 95816;
/// let converted = convert_32bit_account_id_to_64bit(_32bit_id);
///
/// let expected_id = 76561197960361544;
/// assert_eq!(expected_id, converted);
///
/// ```
pub fn convert_32bit_account_id_to_64bit(account_id_32bit: i64) -> i64 {
    let valves_magic_constant = 76561197960265728;
    let mut converted_to_64_bit = account_id_32bit;
    converted_to_64_bit += valves_magic_constant;
    converted_to_64_bit
}


/// Converts given 64 bit Steam account id to 32 bit
///
/// # Examples
///
/// ```
/// use steam_webapi_rust_sdk::convert_64bit_account_id_to_32bit;
///
/// let _64bit_id = 76561197960361544;
/// let converted = convert_64bit_account_id_to_32bit(_64bit_id);
///
/// let expected_id = 95816;
/// assert_eq!(expected_id, converted);
///
/// ```
pub fn convert_64bit_account_id_to_32bit(account_id_32bit: i64) -> i64 {
    let valves_magic_constant = 76561197960265728;
    let mut converted_to_32_bit = account_id_32bit;
    converted_to_32_bit -= valves_magic_constant;
    converted_to_32_bit
}


pub(crate) fn get_scheme() -> String {
    "https".to_string()
}

pub(crate) fn get_host() -> String {
    let host = "api.steampowered.com".to_string();
    host
}

pub(crate) fn make_api_call(url: String) -> Result<String, String> {

    let boxed_response = minreq::get(url).send();
    if boxed_response.is_err() {
        return Err("Operation timed out (API call)".to_string());
    }

    let raw_response : Vec<u8> = boxed_response.unwrap().into_bytes();

    let response_string_boxed = String::from_utf8(raw_response);
    if response_string_boxed.is_err() {
        let error_message = response_string_boxed.err().unwrap().to_string();
        if error_message == "invalid utf-8 sequence of 1 bytes from index 1" {
            return Err("no response from API".to_string());
        }
        return Err("invalid utf-8 sequence".to_string());
    }
    let response_string: String = response_string_boxed.unwrap();

    Ok(response_string)
}