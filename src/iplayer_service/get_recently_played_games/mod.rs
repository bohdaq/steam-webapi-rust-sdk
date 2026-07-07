use std::collections::HashMap;
use serde_json::Value;
use crate::{iplayer_service, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_str, json_u64};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct RecentlyPlayedGame {
    pub appid: u64,
    pub name: String,
    pub playtime_2weeks: u64,
    pub playtime_forever: u64,
    pub img_icon_url: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct RecentlyPlayedGames {
    pub total_count: u64,
    pub games: Vec<RecentlyPlayedGame>,
}

pub fn get_method_name() -> String {
    "GetRecentlyPlayedGames".to_string()
}

pub fn get_version() -> String {
    "v1".to_string()
}

pub fn get(steamid: u64, count: Option<u32>) -> Result<RecentlyPlayedGames, String> {
    let api_url = get_api_url(steamid, count);
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    parse_response(boxed_response.unwrap())
}

pub fn get_api_url(steamid: u64, count: Option<u32>) -> String {
    let interface = iplayer_service::get_interface();
    let method = get_method_name();
    let version = get_version();

    let mut params_map = HashMap::new();
    params_map.insert("steamid".to_string(), steamid.to_string());
    params_map.insert("count".to_string(), count.unwrap_or(0).to_string());
    params_map.insert("key".to_string(), get_steam_web_api_key());

    build_steam_api_url(interface.as_str(), method.as_str(), version.as_str(), params_map)
}

pub fn parse_response(response: String) -> Result<RecentlyPlayedGames, String> {
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
            games.push(RecentlyPlayedGame {
                appid: json_u64(game, "appid"),
                name: json_str(game, "name"),
                playtime_2weeks: json_u64(game, "playtime_2weeks"),
                playtime_forever: json_u64(game, "playtime_forever"),
                img_icon_url: json_str(game, "img_icon_url"),
            });
        }
    }

    Ok(RecentlyPlayedGames {
        total_count: json_u64(result, "total_count"),
        games,
    })
}
