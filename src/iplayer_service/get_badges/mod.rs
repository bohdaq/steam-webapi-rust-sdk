use std::collections::HashMap;
use serde_json::Value;
use crate::{iplayer_service, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_u64};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct Badge {
    pub badgeid: u64,
    pub level: u64,
    pub completion_time: u64,
    pub xp: u64,
    pub scarcity: u64,
    pub appid: u64,
    pub border_color: u64,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Badges {
    pub badges: Vec<Badge>,
    pub player_xp: u64,
    pub player_level: u64,
    pub player_xp_needed_to_level_up: u64,
    pub player_xp_needed_current_level: u64,
}

pub fn get_method_name() -> String {
    "GetBadges".to_string()
}

pub fn get_version() -> String {
    "v1".to_string()
}

pub fn get(steamid: u64) -> Result<Badges, String> {
    let api_url = get_api_url(steamid);
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    parse_response(boxed_response.unwrap())
}

pub fn get_api_url(steamid: u64) -> String {
    let interface = iplayer_service::get_interface();
    let method = get_method_name();
    let version = get_version();

    let mut params_map = HashMap::new();
    params_map.insert("steamid".to_string(), steamid.to_string());
    params_map.insert("key".to_string(), get_steam_web_api_key());

    build_steam_api_url(interface.as_str(), method.as_str(), version.as_str(), params_map)
}

pub fn parse_response(response: String) -> Result<Badges, String> {
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

    let boxed_badges = result.get("badges").and_then(Value::as_array);
    let mut badges = vec![];
    if boxed_badges.is_some() {
        for badge in boxed_badges.unwrap() {
            badges.push(Badge {
                badgeid: json_u64(badge, "badgeid"),
                level: json_u64(badge, "level"),
                completion_time: json_u64(badge, "completion_time"),
                xp: json_u64(badge, "xp"),
                scarcity: json_u64(badge, "scarcity"),
                appid: json_u64(badge, "appid"),
                border_color: json_u64(badge, "border_color"),
            });
        }
    }

    Ok(Badges {
        badges,
        player_xp: json_u64(result, "player_xp"),
        player_level: json_u64(result, "player_level"),
        player_xp_needed_to_level_up: json_u64(result, "player_xp_needed_to_level_up"),
        player_xp_needed_current_level: json_u64(result, "player_xp_needed_current_level"),
    })
}
