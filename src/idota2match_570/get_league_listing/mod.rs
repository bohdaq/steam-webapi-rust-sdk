use std::collections::HashMap;
use serde_json::Value;
use crate::{idota2match_570, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_str, json_u64};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct League {
    pub leagueid: u64,
    pub name: String,
    pub description: String,
    pub tournament_url: String,
}

pub fn get_method_name() -> String {
    "GetLeagueListing".to_string()
}

pub fn get_version() -> String {
    "v1".to_string()
}

pub fn get(language: Option<String>) -> Result<Vec<League>, String> {
    let api_url = get_api_url(language);
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    parse_response(boxed_response.unwrap())
}

pub fn get_api_url(language: Option<String>) -> String {
    let interface = idota2match_570::get_interface();
    let method = get_method_name();
    let version = get_version();

    let mut params_map = HashMap::new();
    if language.is_some() {
        params_map.insert("language".to_string(), language.unwrap());
    }
    params_map.insert("key".to_string(), get_steam_web_api_key());

    build_steam_api_url(interface.as_str(), method.as_str(), version.as_str(), params_map)
}

pub fn parse_response(response: String) -> Result<Vec<League>, String> {
    let boxed_initial_parse = serde_json::from_str(&response);
    if boxed_initial_parse.is_err() {
        return Err(boxed_initial_parse.err().unwrap().to_string());
    }
    let json: Value = boxed_initial_parse.unwrap();

    let boxed_leagues = json.get("result").and_then(|r| r.get("leagues")).and_then(Value::as_array);
    if boxed_leagues.is_none() {
        return Err("response does not contain any leagues".to_string());
    }

    let mut leagues = vec![];
    for league in boxed_leagues.unwrap() {
        leagues.push(League {
            leagueid: json_u64(league, "leagueid"),
            name: json_str(league, "name"),
            description: json_str(league, "description"),
            tournament_url: json_str(league, "tournament_url"),
        });
    }

    Ok(leagues)
}
