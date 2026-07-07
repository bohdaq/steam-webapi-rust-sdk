use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::iplayer_service::get_steam_level::{get_api_url, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url() {
    let api_url = get_api_url(76561197960435530);

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/IPlayerService/GetSteamLevel/v1", components.path);

    let params = components.query.unwrap();
    let boxed_steamid = params.get("steamid");
    assert_eq!(76561197960435530, boxed_steamid.unwrap().parse::<u64>().unwrap());

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn parse() {
    let response = r#"{"response": {"player_level": 22}}"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());
    assert_eq!(22, boxed_parse.unwrap());
}

#[test]
fn parse_missing_result() {
    let boxed_parse = parse_response("{}".to_string());
    assert!(boxed_parse.is_err());
}
