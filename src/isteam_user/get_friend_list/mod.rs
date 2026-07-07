use std::collections::HashMap;
use serde_json::Value;
use crate::{isteam_user, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_str, json_u64};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct Friend {
    pub steamid: String,
    pub relationship: String,
    pub friend_since: u64,
}

pub fn get_method_name() -> String {
    "GetFriendList".to_string()
}

pub fn get_version() -> String {
    "v1".to_string()
}

pub fn get(steamid: u64, relationship: Option<String>) -> Result<Vec<Friend>, String> {
    let api_url = get_api_url(steamid, relationship);
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    parse_response(boxed_response.unwrap())
}

pub fn get_api_url(steamid: u64, relationship: Option<String>) -> String {
    let interface = isteam_user::get_interface();
    let method = get_method_name();
    let version = get_version();

    let mut params_map = HashMap::new();
    params_map.insert("steamid".to_string(), steamid.to_string());
    params_map.insert("relationship".to_string(), relationship.unwrap_or("friend".to_string()));
    params_map.insert("key".to_string(), get_steam_web_api_key());

    build_steam_api_url(interface.as_str(), method.as_str(), version.as_str(), params_map)
}

pub fn parse_response(response: String) -> Result<Vec<Friend>, String> {
    let boxed_initial_parse = serde_json::from_str(&response);
    if boxed_initial_parse.is_err() {
        return Err(boxed_initial_parse.err().unwrap().to_string());
    }
    let json: Value = boxed_initial_parse.unwrap();

    let boxed_friends = json.get("friendslist").and_then(|r| r.get("friends")).and_then(Value::as_array);
    if boxed_friends.is_none() {
        return Err("response does not contain a friends list (the profile's friends list may be private)".to_string());
    }

    let mut friends = vec![];
    for friend in boxed_friends.unwrap() {
        friends.push(Friend {
            steamid: json_str(friend, "steamid"),
            relationship: json_str(friend, "relationship"),
            friend_since: json_u64(friend, "friend_since"),
        });
    }

    Ok(friends)
}
