use std::collections::HashMap;
use serde_json::Value;
use crate::{isteam_user, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_bool, json_str, json_u64};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct PlayerBans {
    pub steamid: String,
    pub community_banned: bool,
    pub vac_banned: bool,
    pub number_of_vac_bans: u64,
    pub days_since_last_ban: u64,
    pub number_of_game_bans: u64,
    pub economy_ban: String,
}

pub fn get_method_name() -> String {
    "GetPlayerBans".to_string()
}

pub fn get_version() -> String {
    "v1".to_string()
}

pub fn get(steamids: Vec<u64>) -> Result<Vec<PlayerBans>, String> {
    let api_url = get_api_url(steamids);
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    parse_response(boxed_response.unwrap())
}

pub fn get_api_url(steamids: Vec<u64>) -> String {
    let interface = isteam_user::get_interface();
    let method = get_method_name();
    let version = get_version();

    let steamids_csv = steamids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");

    let mut params_map = HashMap::new();
    params_map.insert("steamids".to_string(), steamids_csv);
    params_map.insert("key".to_string(), get_steam_web_api_key());

    build_steam_api_url(interface.as_str(), method.as_str(), version.as_str(), params_map)
}

// note: unlike most ISteamUser/IPlayerService responses, this endpoint has no "response" wrapper.
pub fn parse_response(response: String) -> Result<Vec<PlayerBans>, String> {
    let boxed_initial_parse = serde_json::from_str(&response);
    if boxed_initial_parse.is_err() {
        return Err(boxed_initial_parse.err().unwrap().to_string());
    }
    let json: Value = boxed_initial_parse.unwrap();

    let boxed_players = json.get("players").and_then(Value::as_array);
    if boxed_players.is_none() {
        return Err("response does not contain any players".to_string());
    }

    let mut bans = vec![];
    for player in boxed_players.unwrap() {
        bans.push(PlayerBans {
            steamid: json_str(player, "SteamId"),
            community_banned: json_bool(player, "CommunityBanned"),
            vac_banned: json_bool(player, "VACBanned"),
            number_of_vac_bans: json_u64(player, "NumberOfVACBans"),
            days_since_last_ban: json_u64(player, "DaysSinceLastBan"),
            number_of_game_bans: json_u64(player, "NumberOfGameBans"),
            economy_ban: json_str(player, "EconomyBan"),
        });
    }

    Ok(bans)
}
