use std::collections::HashMap;
use serde_json::Value;
use crate::{isteam_user_stats, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_i64, json_str};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct StatSchema {
    pub name: String,
    pub default_value: i64,
    pub display_name: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct AchievementSchema {
    pub name: String,
    pub default_value: i64,
    pub display_name: String,
    pub hidden: bool,
    pub description: String,
    pub icon: String,
    pub icon_gray: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct GameSchema {
    pub game_name: String,
    pub game_version: String,
    pub stats: Vec<StatSchema>,
    pub achievements: Vec<AchievementSchema>,
}

pub fn get_method_name() -> String {
    "GetSchemaForGame".to_string()
}

pub fn get_version() -> String {
    "v2".to_string()
}

pub fn get(appid: i64) -> Result<GameSchema, String> {
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

pub fn parse_response(response: String) -> Result<GameSchema, String> {
    let boxed_initial_parse = serde_json::from_str(&response);
    if boxed_initial_parse.is_err() {
        return Err(boxed_initial_parse.err().unwrap().to_string());
    }
    let json: Value = boxed_initial_parse.unwrap();

    let boxed_game = json.get("game");
    if boxed_game.is_none() {
        return Err("response does not contain a game schema".to_string());
    }
    let game = boxed_game.unwrap();

    let available_game_stats = game.get("availableGameStats");

    let mut stats = vec![];
    let mut achievements = vec![];
    if available_game_stats.is_some() {
        let stats_schema = available_game_stats.unwrap();

        let boxed_stats = stats_schema.get("stats").and_then(Value::as_array);
        if boxed_stats.is_some() {
            for stat in boxed_stats.unwrap() {
                stats.push(StatSchema {
                    name: json_str(stat, "name"),
                    default_value: json_i64(stat, "defaultvalue"),
                    display_name: json_str(stat, "displayName"),
                });
            }
        }

        let boxed_achievements = stats_schema.get("achievements").and_then(Value::as_array);
        if boxed_achievements.is_some() {
            for achievement in boxed_achievements.unwrap() {
                achievements.push(AchievementSchema {
                    name: json_str(achievement, "name"),
                    default_value: json_i64(achievement, "defaultvalue"),
                    display_name: json_str(achievement, "displayName"),
                    hidden: json_i64(achievement, "hidden") == 1,
                    description: json_str(achievement, "description"),
                    icon: json_str(achievement, "icon"),
                    icon_gray: json_str(achievement, "icongray"),
                });
            }
        }
    }

    Ok(GameSchema {
        game_name: json_str(game, "gameName"),
        game_version: json_str(game, "gameVersion"),
        stats,
        achievements,
    })
}
