// curl https://api.steampowered.com/IDOTA2Match_570/GetMatchDetails/v1\?match_id\=1461414523\&key\=1F2709FC907F0DEE1D1EB4787E06B695

use std::collections::HashMap;
use serde_json::Value;
use url_build_parse::{build_url, UrlAuthority, UrlComponents};
use crate::{get_host, get_scheme, idota2match_570, ResponseMatchHistory};
use crate::idota2match_570::get_match_history::{MatchHistory, Participant};
use crate::util::get_steam_web_api_key;

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
}

pub fn get_method_name() -> String {
    "GetMatchDetails".to_string()
}

pub fn get_version() -> String {
    "v1".to_string()
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

pub fn parse_response(response: String) -> Result<MatchResult, String> {
    let match_result = MatchResult{
        radiant_win: false,
        duration: 0,
        pre_game_duration: 0,
        start_time: 0,
        match_id: 0,
        match_seq_num: 0,
        tower_status_radiant: 0,
        tower_status_dire: 0,
        barracks_status_radiant: 0,
        barracks_status_dire: 0,
        cluster: 0,
        first_blood_time: 0,
        lobby_type: 0,
        human_players: 0,
        leagueid: 0,
        positive_votes: 0,
        negative_votes: 0,
        game_mode: 0,
        flags: 0,
        engine: 0,
        radiant_score: 0,
        dire_score: 0,
        players: vec![]
    };

    let boxed_initial_parse = serde_json::from_str(&response);
    if boxed_initial_parse.is_err() {
        return Err(boxed_initial_parse.err().unwrap().to_string());
    }
    let mut json: Value = boxed_initial_parse.unwrap();

    let mut result = json["result".to_string()].take();
    Ok(match_result)

}