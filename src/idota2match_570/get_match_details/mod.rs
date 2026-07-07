// curl https://api.steampowered.com/IDOTA2Match_570/GetMatchDetails/v1\?match_id\=1461414523\&key\=1F2709FC907F0DEE1D1EB4787E06B695

use std::collections::HashMap;
use std::fs;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;
use serde_json::Value;
use url_build_parse::{build_url, UrlAuthority, UrlComponents};
use crate::{get_host, get_scheme, idota2match_570, make_api_call};
use crate::util::{get_cache_dir_path, get_json_filetype, get_steam_web_api_key};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct MatchResult {
    pub radiant_win: bool,
    pub duration: u64,
    pub pre_game_duration: u64,
    pub start_time: u64,
    pub match_id: u64,
    pub match_seq_num: u64,
    pub tower_status_radiant: u64,
    pub tower_status_dire: u64,
    pub barracks_status_radiant: u64,
    pub barracks_status_dire: u64,
    pub cluster: u64,
    pub first_blood_time: u64,
    pub lobby_type: u64,
    pub human_players: u64,
    pub leagueid: u64,
    pub positive_votes: u64,
    pub negative_votes: u64,
    pub game_mode: u64,
    pub flags: u64,
    pub engine: u64,
    pub radiant_score: u64,
    pub dire_score: u64,
    pub players: Vec<PlayerStats>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct PlayerStats {
    pub account_id: u64,
    pub player_slot: u64,
    pub team_number: u64,
    pub team_slot: u64,
    pub hero_id: u64,
    pub item_0: u64,
    pub item_1: u64,
    pub item_2: u64,
    pub item_3: u64,
    pub item_4: u64,
    pub item_5: u64,
    pub backpack_0: u64,
    pub backpack_1: u64,
    pub backpack_2: u64,
    pub item_neutral: u64,
    pub kills: u64,
    pub deaths: u64,
    pub assists: u64,
    pub leaver_status: u64,
    pub last_hits: u64,
    pub denies: u64,
    pub gold_per_min: u64,
    pub xp_per_min: u64,
    pub level: u64,
    pub net_worth: u64,
    pub aghanims_scepter: u64,
    pub aghanims_shard: u64,
    pub moonshard: u64,
}

pub fn get_method_name() -> String {
    "GetMatchDetails".to_string()
}

pub fn get_version() -> String {
    "v1".to_string()
}

/// Retrieves match details for the given match id. Makes an API call to Steam and caches the
/// response, mirroring `store_steampowered_com::appdetails::get`.
pub fn get(match_id: u64) -> Result<MatchResult, String> {
    let api_url = get_api_url(match_id);
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    let response = boxed_response.unwrap();
    parse_response(response, match_id)
}

/// Retrieves match details for the given match id from the local cache. Returns an error if the
/// resource hasn't been cached yet, mirroring `store_steampowered_com::appdetails::get_cached`.
pub fn get_cached(match_id: u64) -> Result<MatchResult, String> {
    let filepath = get_resource_filepath(match_id);

    let boxed_read = read_to_string(filepath);
    if boxed_read.is_ok() {
        let cached_api_response = boxed_read.unwrap();
        parse_response(cached_api_response, match_id)
    } else {
        Err("Cached resource not readable. Consider use get call to retrieve data from steam api".to_string())
    }
}

pub fn get_api_url(match_id: u64) -> String {
    let  interface = idota2match_570::get_interface();
    let  method = get_method_name();
    let  version = get_version();

    let path = [
        "/".to_string(),
        interface, "/".to_string(),
        method, "/".to_string(),
        version
    ].join("");

    let mut params_map = HashMap::new();

    params_map.insert("match_id".to_string(), match_id.to_string());

    params_map.insert("key".to_string(), get_steam_web_api_key());

    let url_builder = UrlComponents{
        scheme: get_scheme(),
        authority: Some(UrlAuthority{
            user_info: None,
            host: get_host(),
            port: None
        }),
        path,
        query: Some(params_map),
        fragment: None
    };

    let url = build_url(url_builder).unwrap();
    url
}

pub fn get_resource_filepath(match_id: u64) -> String {
    let interface = idota2match_570::get_interface();
    let method = get_method_name();
    let version = get_version();

    let resource = [
        interface, "-".to_string(),
        method, "-".to_string(),
        version, "-".to_string(),
        match_id.to_string(), ".".to_string(),
        get_json_filetype()
    ].join("");

    [get_cache_dir_path(), "/".to_string(), resource].join("")
}

fn as_u64(value: &Value, key: &str) -> u64 {
    value.get(key).and_then(Value::as_u64).unwrap_or(0)
}

pub fn parse_response(response: String, match_id: u64) -> Result<MatchResult, String> {
    let boxed_initial_parse = serde_json::from_str(&response);
    if boxed_initial_parse.is_err() {
        return Err(boxed_initial_parse.err().unwrap().to_string());
    }
    let json: Value = boxed_initial_parse.unwrap();

    let boxed_result = json.get("result");
    if boxed_result.is_none() {
        return Err("response does not contain a result".to_string());
    }
    let result = boxed_result.unwrap();

    let boxed_error = result.get("error").and_then(Value::as_str);
    if boxed_error.is_some() {
        return Err(boxed_error.unwrap().to_string());
    }

    let boxed_players = result.get("players").and_then(Value::as_array);
    if boxed_players.is_none() {
        return Err("response does not contain players!".to_string());
    }

    let mut players = vec![];
    for player_result in boxed_players.unwrap() {
        let player_stats = PlayerStats {
            account_id: as_u64(player_result, "account_id"),
            player_slot: as_u64(player_result, "player_slot"),
            team_number: as_u64(player_result, "team_number"),
            team_slot: as_u64(player_result, "team_slot"),
            hero_id: as_u64(player_result, "hero_id"),
            item_0: as_u64(player_result, "item_0"),
            item_1: as_u64(player_result, "item_1"),
            item_2: as_u64(player_result, "item_2"),
            item_3: as_u64(player_result, "item_3"),
            item_4: as_u64(player_result, "item_4"),
            item_5: as_u64(player_result, "item_5"),
            backpack_0: as_u64(player_result, "backpack_0"),
            backpack_1: as_u64(player_result, "backpack_1"),
            backpack_2: as_u64(player_result, "backpack_2"),
            item_neutral: as_u64(player_result, "item_neutral"),
            kills: as_u64(player_result, "kills"),
            deaths: as_u64(player_result, "deaths"),
            assists: as_u64(player_result, "assists"),
            leaver_status: as_u64(player_result, "leaver_status"),
            last_hits: as_u64(player_result, "last_hits"),
            denies: as_u64(player_result, "denies"),
            gold_per_min: as_u64(player_result, "gold_per_min"),
            xp_per_min: as_u64(player_result, "xp_per_min"),
            level: as_u64(player_result, "level"),
            net_worth: as_u64(player_result, "net_worth"),
            aghanims_scepter: as_u64(player_result, "aghanims_scepter"),
            aghanims_shard: as_u64(player_result, "aghanims_shard"),
            moonshard: as_u64(player_result, "moonshard"),
        };
        players.push(player_stats);
    }

    let match_result = MatchResult {
        radiant_win: result.get("radiant_win").and_then(Value::as_bool).unwrap_or(false),
        duration: as_u64(result, "duration"),
        pre_game_duration: as_u64(result, "pre_game_duration"),
        start_time: as_u64(result, "start_time"),
        match_id: as_u64(result, "match_id"),
        match_seq_num: as_u64(result, "match_seq_num"),
        tower_status_radiant: as_u64(result, "tower_status_radiant"),
        tower_status_dire: as_u64(result, "tower_status_dire"),
        barracks_status_radiant: as_u64(result, "barracks_status_radiant"),
        barracks_status_dire: as_u64(result, "barracks_status_dire"),
        cluster: as_u64(result, "cluster"),
        first_blood_time: as_u64(result, "first_blood_time"),
        lobby_type: as_u64(result, "lobby_type"),
        human_players: as_u64(result, "human_players"),
        leagueid: as_u64(result, "leagueid"),
        positive_votes: as_u64(result, "positive_votes"),
        negative_votes: as_u64(result, "negative_votes"),
        game_mode: as_u64(result, "game_mode"),
        flags: as_u64(result, "flags"),
        engine: as_u64(result, "engine"),
        radiant_score: as_u64(result, "radiant_score"),
        dire_score: as_u64(result, "dire_score"),
        players,
    };

    cache_response(match_id, &response);

    Ok(match_result)
}

fn cache_response(match_id: u64, response_string: &str) {
    let cache_dir = get_cache_dir_path();
    if !Path::new(cache_dir.as_str()).is_dir() {
        fs::create_dir_all(cache_dir).unwrap();
    }

    let filepath = get_resource_filepath(match_id);
    let mut file: File = File::create(filepath).unwrap();
    file.write_all(response_string.as_ref()).unwrap();
}
