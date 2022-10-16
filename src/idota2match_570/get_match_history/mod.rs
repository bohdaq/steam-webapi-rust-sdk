use std::collections::HashMap;
use url_build_parse::{build_url, UrlAuthority, UrlComponents};
use crate::{convert_32bit_account_id_to_64bit, get_host, get_scheme, idota2match_570};
use crate::util::{get_cache_dir_path, get_steam_web_api_key};

#[cfg(test)]
mod tests;

/// Returns method name invoked on Steam API.
///
/// # Examples
///
/// ```
/// let method_name = steam_webapi_rust_sdk::isteam_apps::get_match_history::get_method_name();
///
/// assert!(method_name == "GetMatchHistory".to_string());
/// ```
pub fn get_method_name() -> String {
    "GetMatchHistory".to_string()
}

/// Returns version of the method invoked on Steam API.
///
/// # Examples
///
/// ```
/// let version = steam_webapi_rust_sdk::isteam_apps::get_match_history::get_version();
///
/// assert!(version == "v1".to_string());
/// ```
pub fn get_version() -> String {
    "v1".to_string()
}

// curl https://api.steampowered.com/IDOTA2Match_570/GetMatchHistory/v1\?match_id\=664465007\&key\=1F2709FC907F0DEE1D1EB4787E06B695


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

    let path = [interface, "/".to_string(), method, "/".to_string(), version].join("");
    let mut params_map = HashMap::new();

    if account_id.is_some() {
        let account_id_64 = convert_32bit_account_id_to_64bit(account_id.unwrap());
        params_map.insert("account_id".to_string(), account_id_64.to_string());
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

    if tournament_games_only.is_some() {
        params_map.insert("key".to_string(), get_steam_web_api_key());
    }

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

pub struct GameMode {
    none: u8,
    all_pick: u8,
    captains_mode: u8,
    random_draft: u8,
    single_draft: u8,
    all_random: u8,
    intro: u8,
    diretide: u8,
    reverse_captains_mode: u8,
    the_greeviling: u8,
    tutorial: u8,
    mid_only: u8,
    least_played: u8,
    new_player_pool: u8,
    compendium_matchmaking: u8,
    captains_draft: u8,
}

pub const game_modes: GameMode = GameMode {
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

pub struct Skill {
    any: u8,
    normal: u8,
    high: u8,
    very_high: u8,
}

pub const player_skill : Skill = Skill {
    any: 0,
    normal: 1,
    high: 2,
    very_high: 3
};


