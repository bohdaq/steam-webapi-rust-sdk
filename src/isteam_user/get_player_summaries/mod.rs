use std::collections::HashMap;
use serde_json::Value;
use crate::{isteam_user, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_bool, json_str, json_u64};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct PlayerSummary {
    pub steamid: String,
    pub communityvisibilitystate: u64,
    pub profilestate: u64,
    pub personaname: String,
    pub profileurl: String,
    pub avatar: String,
    pub avatarmedium: String,
    pub avatarfull: String,
    pub lastlogoff: u64,
    pub personastate: u64,
    pub realname: String,
    pub primaryclanid: String,
    pub timecreated: u64,
    pub loccountrycode: String,
    pub locstatecode: String,
    pub gameid: String,
    pub gameextrainfo: String,
    pub comment_permission: bool,
}

pub fn get_method_name() -> String {
    "GetPlayerSummaries".to_string()
}

pub fn get_version() -> String {
    "v2".to_string()
}

pub fn get(steamids: Vec<u64>) -> Result<Vec<PlayerSummary>, String> {
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

pub fn parse_response(response: String) -> Result<Vec<PlayerSummary>, String> {
    let boxed_initial_parse = serde_json::from_str(&response);
    if boxed_initial_parse.is_err() {
        return Err(boxed_initial_parse.err().unwrap().to_string());
    }
    let json: Value = boxed_initial_parse.unwrap();

    let boxed_players = json.get("response").and_then(|r| r.get("players")).and_then(Value::as_array);
    if boxed_players.is_none() {
        return Err("response does not contain any players".to_string());
    }

    let mut players = vec![];
    for player in boxed_players.unwrap() {
        players.push(PlayerSummary {
            steamid: json_str(player, "steamid"),
            communityvisibilitystate: json_u64(player, "communityvisibilitystate"),
            profilestate: json_u64(player, "profilestate"),
            personaname: json_str(player, "personaname"),
            profileurl: json_str(player, "profileurl"),
            avatar: json_str(player, "avatar"),
            avatarmedium: json_str(player, "avatarmedium"),
            avatarfull: json_str(player, "avatarfull"),
            lastlogoff: json_u64(player, "lastlogoff"),
            personastate: json_u64(player, "personastate"),
            realname: json_str(player, "realname"),
            primaryclanid: json_str(player, "primaryclanid"),
            timecreated: json_u64(player, "timecreated"),
            loccountrycode: json_str(player, "loccountrycode"),
            locstatecode: json_str(player, "locstatecode"),
            gameid: json_str(player, "gameid"),
            gameextrainfo: json_str(player, "gameextrainfo"),
            comment_permission: json_u64(player, "commentpermission") == 1 || json_bool(player, "commentpermission"),
        });
    }

    Ok(players)
}
