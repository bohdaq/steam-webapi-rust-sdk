use std::collections::HashMap;
use serde_json::Value;
use crate::{isteam_user, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_str, json_u64};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct VanityUrlResolution {
    pub success: u64,
    pub steamid: String,
    pub message: String,
}

pub fn get_method_name() -> String {
    "ResolveVanityURL".to_string()
}

pub fn get_version() -> String {
    "v1".to_string()
}

pub fn get(vanity_url: String, url_type: Option<u8>) -> Result<VanityUrlResolution, String> {
    let api_url = get_api_url(vanity_url, url_type);
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    parse_response(boxed_response.unwrap())
}

pub fn get_api_url(vanity_url: String, url_type: Option<u8>) -> String {
    let interface = isteam_user::get_interface();
    let method = get_method_name();
    let version = get_version();

    let mut params_map = HashMap::new();
    params_map.insert("vanityurl".to_string(), vanity_url);
    if url_type.is_some() {
        params_map.insert("url_type".to_string(), url_type.unwrap().to_string());
    }
    params_map.insert("key".to_string(), get_steam_web_api_key());

    build_steam_api_url(interface.as_str(), method.as_str(), version.as_str(), params_map)
}

pub fn parse_response(response: String) -> Result<VanityUrlResolution, String> {
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

    Ok(VanityUrlResolution {
        success: json_u64(result, "success"),
        steamid: json_str(result, "steamid"),
        message: json_str(result, "message"),
    })
}
