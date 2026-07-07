use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::isteam_user_stats::get_number_of_current_players::{get_api_url, parse_response};

#[test]
fn api_url() {
    let api_url = get_api_url(570);

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/ISteamUserStats/GetNumberOfCurrentPlayers/v1", components.path);

    let params = components.query.unwrap();
    let boxed_appid = params.get("appid");
    assert_eq!(570, boxed_appid.unwrap().parse::<i64>().unwrap());
}

#[test]
fn parse() {
    let response = r#"{"response": {"player_count": 500000, "result": 1}}"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());
    assert_eq!(500000, boxed_parse.unwrap());
}

#[test]
fn parse_failed_result() {
    let response = r#"{"response": {"result": 42}}"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_err());
}
