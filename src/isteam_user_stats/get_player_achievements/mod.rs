use std::collections::HashMap;
use serde_json::Value;
use crate::{isteam_user_stats, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_bool, json_str, json_u64};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct PlayerAchievement {
    pub apiname: String,
    pub achieved: bool,
    pub unlocktime: u64,
    pub name: String,
    pub description: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct PlayerAchievements {
    pub steamid: String,
    pub game_name: String,
    pub achievements: Vec<PlayerAchievement>,
}

pub fn get_method_name() -> String {
    "GetPlayerAchievements".to_string()
}

pub fn get_version() -> String {
    "v1".to_string()
}

pub fn get(steamid: u64, appid: i64, language: Option<String>) -> Result<PlayerAchievements, String> {
    let api_url = get_api_url(steamid, appid, language);
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    parse_response(boxed_response.unwrap())
}

pub fn get_api_url(steamid: u64, appid: i64, language: Option<String>) -> String {
    let interface = isteam_user_stats::get_interface();
    let method = get_method_name();
    let version = get_version();

    let mut params_map = HashMap::new();
    params_map.insert("steamid".to_string(), steamid.to_string());
    params_map.insert("appid".to_string(), appid.to_string());
    if language.is_some() {
        params_map.insert("l".to_string(), language.unwrap());
    }
    params_map.insert("key".to_string(), get_steam_web_api_key());

    build_steam_api_url(interface.as_str(), method.as_str(), version.as_str(), params_map)
}

pub fn parse_response(response: String) -> Result<PlayerAchievements, String> {
    let boxed_initial_parse = serde_json::from_str(&response);
    if boxed_initial_parse.is_err() {
        return Err(boxed_initial_parse.err().unwrap().to_string());
    }
    let json: Value = boxed_initial_parse.unwrap();

    let boxed_playerstats = json.get("playerstats");
    if boxed_playerstats.is_none() {
        return Err("response does not contain playerstats".to_string());
    }
    let playerstats = boxed_playerstats.unwrap();

    let boxed_success = playerstats.get("success").and_then(Value::as_bool);
    if boxed_success == Some(false) {
        let error_message = json_str(playerstats, "error");
        return Err(if error_message.is_empty() { "steam api returned unsuccessful response".to_string() } else { error_message });
    }

    let boxed_achievements = playerstats.get("achievements").and_then(Value::as_array);
    let mut achievements = vec![];
    if boxed_achievements.is_some() {
        for achievement in boxed_achievements.unwrap() {
            achievements.push(PlayerAchievement {
                apiname: json_str(achievement, "apiname"),
                achieved: json_u64(achievement, "achieved") == 1 || json_bool(achievement, "achieved"),
                unlocktime: json_u64(achievement, "unlocktime"),
                name: json_str(achievement, "name"),
                description: json_str(achievement, "description"),
            });
        }
    }

    Ok(PlayerAchievements {
        steamid: json_str(playerstats, "steamID"),
        game_name: json_str(playerstats, "gameName"),
        achievements,
    })
}
