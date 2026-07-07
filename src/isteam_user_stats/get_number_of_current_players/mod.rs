use std::collections::HashMap;
use serde_json::Value;
use crate::{isteam_user_stats, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_i64};

#[cfg(test)]
mod tests;

pub fn get_method_name() -> String {
    "GetNumberOfCurrentPlayers".to_string()
}

pub fn get_version() -> String {
    "v1".to_string()
}

/// Does not require a Steam Web API key.
pub fn get(appid: i64) -> Result<i64, String> {
    let api_url = get_api_url(appid);
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    parse_response(boxed_response.unwrap())
}

pub fn get_api_url(appid: i64) -> String {
    let interface = isteam_user_stats::get_interface();
    let method = get_method_name();
    let version = get_version();

    let mut params_map = HashMap::new();
    params_map.insert("appid".to_string(), appid.to_string());
    params_map.insert("key".to_string(), get_steam_web_api_key());

    build_steam_api_url(interface.as_str(), method.as_str(), version.as_str(), params_map)
}

const RESULT_SUCCESS: i64 = 1;

pub fn parse_response(response: String) -> Result<i64, String> {
    let boxed_initial_parse = serde_json::from_str(&response);
    if boxed_initial_parse.is_err() {
        return Err(boxed_initial_parse.err().unwrap().to_string());
    }
    let json: Value = boxed_initial_parse.unwrap();

    let boxed_result = json.get("response");
    if boxed_result.is_none() {
        return Err("response does not contain a result".to_string());
    }
    let result = boxed_result.unwrap();

    if json_i64(result, "result") != RESULT_SUCCESS {
        return Err("steampowered api returned failed response".to_string());
    }

    Ok(json_i64(result, "player_count"))
}
