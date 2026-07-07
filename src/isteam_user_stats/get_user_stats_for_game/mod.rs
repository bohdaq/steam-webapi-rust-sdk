use std::collections::HashMap;
use serde_json::Value;
use crate::{isteam_user_stats, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_bool, json_f64, json_str, json_u64};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct GameStat {
    pub name: String,
    pub value: f64,
}

#[derive(PartialEq, Clone, Debug)]
pub struct GameAchievement {
    pub name: String,
    pub achieved: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub struct UserStatsForGame {
    pub steamid: String,
    pub game_name: String,
    pub stats: Vec<GameStat>,
    pub achievements: Vec<GameAchievement>,
}

pub fn get_method_name() -> String {
    "GetUserStatsForGame".to_string()
}

pub fn get_version() -> String {
    "v2".to_string()
}

pub fn get(steamid: u64, appid: i64) -> Result<UserStatsForGame, String> {
    let api_url = get_api_url(steamid, appid);
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    parse_response(boxed_response.unwrap())
}

pub fn get_api_url(steamid: u64, appid: i64) -> String {
    let interface = isteam_user_stats::get_interface();
    let method = get_method_name();
    let version = get_version();

    let mut params_map = HashMap::new();
    params_map.insert("steamid".to_string(), steamid.to_string());
    params_map.insert("appid".to_string(), appid.to_string());
    params_map.insert("key".to_string(), get_steam_web_api_key());

    build_steam_api_url(interface.as_str(), method.as_str(), version.as_str(), params_map)
}

pub fn parse_response(response: String) -> Result<UserStatsForGame, String> {
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

    let boxed_stats = playerstats.get("stats").and_then(Value::as_array);
    let mut stats = vec![];
    if boxed_stats.is_some() {
        for stat in boxed_stats.unwrap() {
            stats.push(GameStat {
                name: json_str(stat, "name"),
                value: json_f64(stat, "value"),
            });
        }
    }

    let boxed_achievements = playerstats.get("achievements").and_then(Value::as_array);
    let mut achievements = vec![];
    if boxed_achievements.is_some() {
        for achievement in boxed_achievements.unwrap() {
            achievements.push(GameAchievement {
                name: json_str(achievement, "name"),
                achieved: json_u64(achievement, "achieved") == 1 || json_bool(achievement, "achieved"),
            });
        }
    }

    Ok(UserStatsForGame {
        steamid: json_str(playerstats, "steamID"),
        game_name: json_str(playerstats, "gameName"),
        stats,
        achievements,
    })
}
