use std::collections::HashMap;
use serde_json::Value;
use crate::{isteam_user_stats, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_f64, json_str};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct AchievementPercentage {
    pub name: String,
    pub percent: f64,
}

pub fn get_method_name() -> String {
    "GetGlobalAchievementPercentagesForApp".to_string()
}

pub fn get_version() -> String {
    "v2".to_string()
}

/// Does not require a Steam Web API key.
pub fn get(gameid: i64) -> Result<Vec<AchievementPercentage>, String> {
    let api_url = get_api_url(gameid);
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    parse_response(boxed_response.unwrap())
}

pub fn get_api_url(gameid: i64) -> String {
    let interface = isteam_user_stats::get_interface();
    let method = get_method_name();
    let version = get_version();

    let mut params_map = HashMap::new();
    params_map.insert("gameid".to_string(), gameid.to_string());
    params_map.insert("key".to_string(), get_steam_web_api_key());

    build_steam_api_url(interface.as_str(), method.as_str(), version.as_str(), params_map)
}

pub fn parse_response(response: String) -> Result<Vec<AchievementPercentage>, String> {
    let boxed_initial_parse = serde_json::from_str(&response);
    if boxed_initial_parse.is_err() {
        return Err(boxed_initial_parse.err().unwrap().to_string());
    }
    let json: Value = boxed_initial_parse.unwrap();

    let boxed_achievements = json.get("achievementpercentages").and_then(|r| r.get("achievements")).and_then(Value::as_array);
    if boxed_achievements.is_none() {
        return Err("response does not contain achievement percentages".to_string());
    }

    let mut achievements = vec![];
    for achievement in boxed_achievements.unwrap() {
        achievements.push(AchievementPercentage {
            name: json_str(achievement, "name"),
            percent: json_f64(achievement, "percent"),
        });
    }

    Ok(achievements)
}
