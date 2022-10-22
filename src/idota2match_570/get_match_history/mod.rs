use std::collections::HashMap;
use serde::Serialize;
use serde::Deserialize;
use serde_json::Value;
use url_build_parse::{build_url, UrlAuthority, UrlComponents};
use crate::{get_host, get_scheme, idota2match_570, make_api_call};
use crate::util::{get_steam_web_api_key};

#[cfg(test)]
mod tests;

// curl https://api.steampowered.com/IDOTA2Match_570/GetMatchHistory/v1\?match_id\=664465007\&key\=1F2709FC907F0DEE1D1EB4787E06B695

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ResponseMatchHistory {
    pub status: i64,
    pub num_results: i64,
    pub total_results: i64,
    pub results_remaining: i64,
    pub matches: Vec<MatchHistory>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Participant {
    pub account_id: i64, //steam returns 32bit version
    pub player_slot: i64,
    pub team_number: i64,
    pub team_slot: u8,
    pub hero_id: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct MatchHistory {
    pub match_id: i64,
    pub match_seq_num: i64,
    pub start_time: i64,
    pub lobby_type: i64,
    pub radiant_team_id: i64,
    pub dire_team_id: i64,
    pub players: Vec<Participant>,
}

pub fn get_method_name() -> String {
    "GetMatchHistory".to_string()
}

pub fn get_version() -> String {
    "v1".to_string()
}

pub fn get(account_id: Option<i64>,
           game_mode: Option<u8>,
           skill: Option<u8>,
           min_players: Option<u32>,
           start_at_match_id: Option<u64>,
           matches_requested: Option<u32>,
           tournament_games_only: Option<bool>)
    -> Result<ResponseMatchHistory, String> {
    let api_url = get_api_url(
                            account_id,
                            game_mode,
                            skill,
                            min_players,
                            start_at_match_id,
                            matches_requested,
                            tournament_games_only
    );
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    let response = boxed_response.unwrap();
    let boxed_result = parse_response(response);
    if boxed_result.is_err() {
        return Err(boxed_result.err().unwrap());
    }

    let result = boxed_result.unwrap();
    Ok(result)
}

pub fn get_api_url(account_id: Option<i64>,
                             game_mode: Option<u8>,
                             skill: Option<u8>,
                             min_players: Option<u32>,
                             start_at_match_id: Option<u64>,
                             matches_requested: Option<u32>,
                             tournament_games_only: Option<bool>) -> String {


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

    if account_id.is_some() {
        params_map.insert("account_id".to_string(), account_id.unwrap().to_string());
    }

    if game_mode.is_some() {
        params_map.insert("game_mode".to_string(), game_mode.unwrap().to_string());
    }

    if skill.is_some() {
        params_map.insert("skill".to_string(), skill.unwrap().to_string());
    }

    if min_players.is_some() {
        params_map.insert("min_players".to_string(), min_players.unwrap().to_string());
    }

    if start_at_match_id.is_some() {
        params_map.insert("start_at_match_id".to_string(), start_at_match_id.unwrap().to_string());
    }

    if matches_requested.is_some() {
        params_map.insert("matches_requested".to_string(), matches_requested.unwrap().to_string());
    }

    if tournament_games_only.is_some() {
        params_map.insert("tournament_games_only".to_string(), tournament_games_only.unwrap().to_string());
    }

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

const MATCH_HISTORY_IS_NOT_ALLOWED_BY_USER_PREFERENCES: u8 = 15;

pub fn parse_response(response: String) -> Result<ResponseMatchHistory, String> {

    let boxed_initial_parse = serde_json::from_str(&response);
    if boxed_initial_parse.is_err() {
        return Err(boxed_initial_parse.err().unwrap().to_string());
    }
    let mut json: Value = boxed_initial_parse.unwrap();

    let mut result = json["result".to_string()].take();
    let status = result["status".to_string()].take().as_i64().unwrap();
    if status as u8 == MATCH_HISTORY_IS_NOT_ALLOWED_BY_USER_PREFERENCES {
        return Err("Cannot get match history for a user that hasn't allowed it".to_string())
    }

    let num_results_clone = result["num_results".to_string()].clone();
    let mut num_results = 0;
    if num_results_clone.as_i64().is_some() {
        num_results = num_results_clone.as_i64().unwrap();
    }

    let total_results_clone = result["total_results".to_string()].clone();
    let mut total_results = 0;
    if total_results_clone.as_i64().is_some() {
        total_results = total_results_clone.as_i64().unwrap();
    }

    let results_remaining_clone = result["results_remaining".to_string()].clone();
    let mut results_remaining = 0;
    if results_remaining_clone.as_i64().is_some() {
        results_remaining = results_remaining_clone.as_i64().unwrap();
    }

    let matches_clone = result["matches".to_string()].clone();
    if matches_clone.as_array().is_none() {
        return Err("response does not contain any matches!".to_string())
    }
    let matches = result["matches".to_string()].as_array().unwrap();

    let mut match_history_result = ResponseMatchHistory {
        status,
        num_results,
        total_results,
        results_remaining,
        matches: vec![]
    };

    for match_result in matches {
        let match_id_clone = match_result["match_id"].clone();
        let mut match_id = 0;
        if match_id_clone.as_i64().is_some() {
            match_id = match_id_clone.as_i64().unwrap();
        }

        let match_seq_num_clone = match_result["match_seq_num"].clone();
        let mut match_seq_num = 0;
        if match_seq_num_clone.as_i64().is_some() {
            match_seq_num = match_seq_num_clone.as_i64().unwrap();
        }

        let start_time_clone = match_result["start_time"].clone();
        let mut start_time = 0;
        if start_time_clone.as_i64().is_some() {
            start_time = start_time_clone.as_i64().unwrap();
        }

        let lobby_type_clone = match_result["lobby_type"].clone();
        let mut lobby_type = 0;
        if lobby_type_clone.as_i64().is_some() {
            lobby_type = lobby_type_clone.as_i64().unwrap();
        }

        let radiant_team_id_clone = match_result["radiant_team_id"].clone();
        let mut radiant_team_id = 0;
        if radiant_team_id_clone.as_i64().is_some() {
            radiant_team_id = radiant_team_id_clone.as_i64().unwrap();
        }

        let dire_team_id_clone = match_result["dire_team_id"].clone();
        let mut dire_team_id = 0;
        if dire_team_id_clone.as_i64().is_some() {
            dire_team_id = dire_team_id_clone.as_i64().unwrap();
        }

        let players_clone = match_result["players"].clone();
        if players_clone.as_array().is_none() {
            return Err("response does not contain players!".to_string())
        }
        let players  = players_clone.as_array().unwrap();


        let mut match_history = MatchHistory{
            match_id,
            match_seq_num,
            start_time,
            lobby_type,
            radiant_team_id,
            dire_team_id,
            players: vec![]
        };


        for player_result in players {
            let account_id_clone = player_result["account_id"].clone();
            let mut account_id = 0;
            if account_id_clone.as_i64().is_some() {
                account_id = account_id_clone.as_i64().unwrap();
            }

            let player_slot_clone = player_result["player_slot"].clone();
            let mut player_slot = 0;
            if player_slot_clone.as_i64().is_some() {
                player_slot = player_slot_clone.as_i64().unwrap();
            }

            let team_number_clone = player_result["team_number"].clone();
            let mut team_number = 0;
            if team_number_clone.as_i64().is_some() {
                team_number = team_number_clone.as_i64().unwrap();
            }

            let team_slot_clone = player_result["team_slot"].clone();
            let mut team_slot_i64 = 0;
            if team_slot_clone.as_i64().is_some() {
                team_slot_i64 = team_slot_clone.as_i64().unwrap();
            }

            let hero_id_clone = player_result["hero_id"].clone();
            let mut hero_id = 0;
            if hero_id_clone.as_i64().is_some() {
                hero_id = hero_id_clone.as_i64().unwrap();
            }



            let team_slot : u8 = team_slot_i64 as u8;

            let player = Participant{
                account_id,
                player_slot,
                team_number,
                team_slot,
                hero_id
            };

            match_history.players.push(player);
        }

        match_history_result.matches.push(match_history)
    }



    Ok(match_history_result)
}

pub const GAME_MODE: GameMode = GameMode{
    none: 0,
    all_pick: 1,
    captains_mode: 2,
    random_draft: 3,
    single_draft: 4,
    all_random: 5,
    intro: 6,
    diretide: 7,
    reverse_captains_mode: 8,
    the_greeviling: 9,
    tutorial: 10,
    mid_only: 11,
    least_played: 12,
    new_player_pool: 13,
    compendium_matchmaking: 14,
    captains_draft: 16
};

pub struct GameMode {
    pub none: u8,
    pub all_pick: u8,
    pub captains_mode: u8,
    pub random_draft: u8,
    pub single_draft: u8,
    pub all_random: u8,
    pub intro: u8,
    pub diretide: u8,
    pub reverse_captains_mode: u8,
    pub the_greeviling: u8,
    pub tutorial: u8,
    pub mid_only: u8,
    pub least_played: u8,
    pub new_player_pool: u8,
    pub compendium_matchmaking: u8,
    pub captains_draft: u8,
}

pub struct Skill {
    pub any: u8,
    pub normal: u8,
    pub high: u8,
    pub very_high: u8,
}

pub const PLAYER_SKILL : Skill = Skill {
    any: 0,
    normal: 1,
    high: 2,
    very_high: 3
};

pub struct LobbyType {
    pub invalid: i8,
    pub public_matchmaking: u8,
    pub practise: u8,
    pub tournament: u8,
    pub tutorial: u8,
    pub co_op_with_bots: u8,
    pub team_match: u8,
    pub solo_queue: u8,
    pub ranked_matchmaking: u8,
    pub one_vs_one_solo_mid: u8,
}

pub const LOBBY_TYPE: LobbyType = LobbyType {
    invalid: -1,
    public_matchmaking: 0,
    practise: 1,
    tournament: 2,
    tutorial: 3,
    co_op_with_bots: 4,
    team_match: 5,
    solo_queue: 6,
    ranked_matchmaking: 7,
    one_vs_one_solo_mid: 8
};
