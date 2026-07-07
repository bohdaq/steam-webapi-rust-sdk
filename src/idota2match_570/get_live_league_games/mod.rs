use std::collections::HashMap;
use serde_json::Value;
use crate::{idota2match_570, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_str, json_u64};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct LivePlayer {
    pub account_id: u64,
    pub hero_id: u64,
    pub team: u64,
}

#[derive(PartialEq, Clone, Debug)]
pub struct LiveTeam {
    pub team_name: String,
    pub team_id: u64,
    pub team_logo: u64,
    pub complete: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub struct LiveLeagueGame {
    pub players: Vec<LivePlayer>,
    pub radiant_team: LiveTeam,
    pub dire_team: LiveTeam,
    pub lobby_id: u64,
    pub match_id: u64,
    pub spectators: u64,
    pub league_id: u64,
    pub series_type: u64,
    pub radiant_series_wins: u64,
    pub dire_series_wins: u64,
}

pub fn get_method_name() -> String {
    "GetLiveLeagueGames".to_string()
}

pub fn get_version() -> String {
    "v1".to_string()
}

pub fn get() -> Result<Vec<LiveLeagueGame>, String> {
    let api_url = get_api_url();
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    parse_response(boxed_response.unwrap())
}

pub fn get_api_url() -> String {
    let interface = idota2match_570::get_interface();
    let method = get_method_name();
    let version = get_version();

    let mut params_map = HashMap::new();
    params_map.insert("key".to_string(), get_steam_web_api_key());

    build_steam_api_url(interface.as_str(), method.as_str(), version.as_str(), params_map)
}

fn parse_team(team: Option<&Value>) -> LiveTeam {
    let mut result = LiveTeam { team_name: "".to_string(), team_id: 0, team_logo: 0, complete: false };

    if team.is_some() {
        let team_value = team.unwrap();
        result.team_name = json_str(team_value, "team_name");
        result.team_id = json_u64(team_value, "team_id");
        result.team_logo = json_u64(team_value, "team_logo");
        result.complete = json_u64(team_value, "complete") == 1;
    }

    result
}

pub fn parse_response(response: String) -> Result<Vec<LiveLeagueGame>, String> {
    let boxed_initial_parse = serde_json::from_str(&response);
    if boxed_initial_parse.is_err() {
        return Err(boxed_initial_parse.err().unwrap().to_string());
    }
    let json: Value = boxed_initial_parse.unwrap();

    let boxed_games = json.get("result").and_then(|r| r.get("games")).and_then(Value::as_array);
    if boxed_games.is_none() {
        return Err("response does not contain any games".to_string());
    }

    let mut games = vec![];
    for game in boxed_games.unwrap() {
        let boxed_players = game.get("players").and_then(Value::as_array);
        let mut players = vec![];
        if boxed_players.is_some() {
            for player in boxed_players.unwrap() {
                players.push(LivePlayer {
                    account_id: json_u64(player, "account_id"),
                    hero_id: json_u64(player, "hero_id"),
                    team: json_u64(player, "team"),
                });
            }
        }

        games.push(LiveLeagueGame {
            players,
            radiant_team: parse_team(game.get("radiant_team")),
            dire_team: parse_team(game.get("dire_team")),
            lobby_id: json_u64(game, "lobby_id"),
            match_id: json_u64(game, "match_id"),
            spectators: json_u64(game, "spectators"),
            league_id: json_u64(game, "league_id"),
            series_type: json_u64(game, "series_type"),
            radiant_series_wins: json_u64(game, "radiant_series_wins"),
            dire_series_wins: json_u64(game, "dire_series_wins"),
        });
    }

    Ok(games)
}
