use std::collections::HashMap;
use serde_json::Value;
use crate::{isteam_news, make_api_call};
use crate::util::{build_steam_api_url, get_steam_web_api_key, json_bool, json_str, json_u64};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug)]
pub struct NewsItem {
    pub gid: String,
    pub title: String,
    pub url: String,
    pub is_external_url: bool,
    pub author: String,
    pub contents: String,
    pub feedlabel: String,
    pub date: u64,
    pub feedname: String,
}

pub fn get_method_name() -> String {
    "GetNewsForApp".to_string()
}

pub fn get_version() -> String {
    "v2".to_string()
}

/// Does not require a Steam Web API key.
pub fn get(appid: i64, count: Option<u32>, maxlength: Option<u32>) -> Result<Vec<NewsItem>, String> {
    let api_url = get_api_url(appid, count, maxlength);
    let boxed_response = make_api_call(api_url);
    if boxed_response.is_err() {
        return Err(boxed_response.err().unwrap());
    }

    parse_response(boxed_response.unwrap())
}

pub fn get_api_url(appid: i64, count: Option<u32>, maxlength: Option<u32>) -> String {
    let interface = isteam_news::get_interface();
    let method = get_method_name();
    let version = get_version();

    let mut params_map = HashMap::new();
    params_map.insert("appid".to_string(), appid.to_string());
    if count.is_some() {
        params_map.insert("count".to_string(), count.unwrap().to_string());
    }
    if maxlength.is_some() {
        params_map.insert("maxlength".to_string(), maxlength.unwrap().to_string());
    }
    params_map.insert("key".to_string(), get_steam_web_api_key());

    build_steam_api_url(interface.as_str(), method.as_str(), version.as_str(), params_map)
}

pub fn parse_response(response: String) -> Result<Vec<NewsItem>, String> {
    let boxed_initial_parse = serde_json::from_str(&response);
    if boxed_initial_parse.is_err() {
        return Err(boxed_initial_parse.err().unwrap().to_string());
    }
    let json: Value = boxed_initial_parse.unwrap();

    let boxed_newsitems = json.get("appnews").and_then(|r| r.get("newsitems")).and_then(Value::as_array);
    if boxed_newsitems.is_none() {
        return Err("response does not contain any news items".to_string());
    }

    let mut newsitems = vec![];
    for item in boxed_newsitems.unwrap() {
        newsitems.push(NewsItem {
            gid: json_str(item, "gid"),
            title: json_str(item, "title"),
            url: json_str(item, "url"),
            is_external_url: json_bool(item, "is_external_url"),
            author: json_str(item, "author"),
            contents: json_str(item, "contents"),
            feedlabel: json_str(item, "feedlabel"),
            date: json_u64(item, "date"),
            feedname: json_str(item, "feedname"),
        });
    }

    Ok(newsitems)
}
