use std::collections::HashMap;
use serde_json::Value;
use crate::{idota2match_570, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_str, json_u64};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct TeamInfo {
    pub team_id: u64,
    pub name: String,
    pub tag: String,
    pub time_created: u64,
    pub logo: u64,
    pub logo_sponsor: u64,
    pub country_code: String,
    pub url: String,
}

pub fn get_method_name() -> String {
    "GetTeamInfoByTeamID".to_string()
}

pub fn get_version() -> String {
    "v1".to_string()
}

pub fn get(start_at_team_id: Option<u64>, teams_requested: Option<u32>) -> Result<Vec<TeamInfo>, String> {
    let api_url = get_api_url(start_at_team_id, teams_requested);
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    parse_response(boxed_response.unwrap())
}

pub fn get_api_url(start_at_team_id: Option<u64>, teams_requested: Option<u32>) -> String {
    let interface = idota2match_570::get_interface();
    let method = get_method_name();
    let version = get_version();

    let mut params_map = HashMap::new();
    if start_at_team_id.is_some() {
        params_map.insert("start_at_team_id".to_string(), start_at_team_id.unwrap().to_string());
    }
    if teams_requested.is_some() {
        params_map.insert("teams_requested".to_string(), teams_requested.unwrap().to_string());
    }
    params_map.insert("key".to_string(), get_steam_web_api_key());

    build_steam_api_url(interface.as_str(), method.as_str(), version.as_str(), params_map)
}

pub fn parse_response(response: String) -> Result<Vec<TeamInfo>, String> {
    let boxed_initial_parse = serde_json::from_str(&response);
    if boxed_initial_parse.is_err() {
        return Err(boxed_initial_parse.err().unwrap().to_string());
    }
    let json: Value = boxed_initial_parse.unwrap();

    let boxed_teams = json.get("result").and_then(|r| r.get("teams")).and_then(Value::as_array);
    if boxed_teams.is_none() {
        return Err("response does not contain any teams".to_string());
    }

    let mut teams = vec![];
    for team in boxed_teams.unwrap() {
        teams.push(TeamInfo {
            team_id: json_u64(team, "team_id"),
            name: json_str(team, "name"),
            tag: json_str(team, "tag"),
            time_created: json_u64(team, "time_created"),
            logo: json_u64(team, "logo"),
            logo_sponsor: json_u64(team, "logo_sponsor"),
            country_code: json_str(team, "country_code"),
            url: json_str(team, "url"),
        });
    }

    Ok(teams)
}
