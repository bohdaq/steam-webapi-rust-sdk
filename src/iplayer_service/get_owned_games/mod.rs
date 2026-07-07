use std::collections::HashMap;
use serde_json::Value;
use crate::{iplayer_service, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_bool, json_str, json_u64};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct OwnedGame {
    pub appid: u64,
    pub name: String,
    pub playtime_forever: u64,
    pub img_icon_url: String,
    pub has_community_visible_stats: bool,
    pub playtime_windows_forever: u64,
    pub playtime_mac_forever: u64,
    pub playtime_linux_forever: u64,
    pub rtime_last_played: u64,
}

#[derive(PartialEq, Clone, Debug)]
pub struct OwnedGames {
    pub game_count: u64,
    pub games: Vec<OwnedGame>,
}

pub fn get_method_name() -> String {
    "GetOwnedGames".to_string()
}

pub fn get_version() -> String {
    "v1".to_string()
}

pub fn get(steamid: u64, include_appinfo: Option<bool>, include_played_free_games: Option<bool>) -> Result<OwnedGames, String> {
    let api_url = get_api_url(steamid, include_appinfo, include_played_free_games);
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    parse_response(boxed_response.unwrap())
}

pub fn get_api_url(steamid: u64, include_appinfo: Option<bool>, include_played_free_games: Option<bool>) -> String {
    let interface = iplayer_service::get_interface();
    let method = get_method_name();
    let version = get_version();

    let mut params_map = HashMap::new();
    params_map.insert("steamid".to_string(), steamid.to_string());
    params_map.insert("include_appinfo".to_string(), include_appinfo.unwrap_or(false).to_string());
    params_map.insert("include_played_free_games".to_string(), include_played_free_games.unwrap_or(false).to_string());
    params_map.insert("key".to_string(), get_steam_web_api_key());

    build_steam_api_url(interface.as_str(), method.as_str(), version.as_str(), params_map)
}

pub fn parse_response(response: String) -> Result<OwnedGames, String> {
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

    let boxed_games = result.get("games").and_then(Value::as_array);
    let mut games = vec![];
    if boxed_games.is_some() {
        for game in boxed_games.unwrap() {
            games.push(OwnedGame {
                appid: json_u64(game, "appid"),
                name: json_str(game, "name"),
                playtime_forever: json_u64(game, "playtime_forever"),
                img_icon_url: json_str(game, "img_icon_url"),
                has_community_visible_stats: json_bool(game, "has_community_visible_stats"),
                playtime_windows_forever: json_u64(game, "playtime_windows_forever"),
                playtime_mac_forever: json_u64(game, "playtime_mac_forever"),
                playtime_linux_forever: json_u64(game, "playtime_linux_forever"),
                rtime_last_played: json_u64(game, "rtime_last_played"),
            });
        }
    }

    Ok(OwnedGames {
        game_count: json_u64(result, "game_count"),
        games,
    })
}
