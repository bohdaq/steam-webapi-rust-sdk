use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::iplayer_service::get_badges::{get_api_url, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url() {
    let api_url = get_api_url(76561197960435530);

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/IPlayerService/GetBadges/v1", components.path);

    let params = components.query.unwrap();
    let boxed_steamid = params.get("steamid");
    assert_eq!(76561197960435530, boxed_steamid.unwrap().parse::<u64>().unwrap());

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn parse() {
    let response = r#"{
        "response": {
            "badges": [
                {
                    "badgeid": 1,
                    "level": 6,
                    "completion_time": 1234567890,
                    "xp": 800,
                    "scarcity": 317255,
                    "appid": 440,
                    "border_color": 0
                }
            ],
            "player_xp": 7401,
            "player_level": 22,
            "player_xp_needed_to_level_up": 400,
            "player_xp_needed_current_level": 7300
        }
    }"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let badges = boxed_parse.unwrap();
    assert_eq!(1, badges.badges.len());
    assert_eq!(7401, badges.player_xp);
    assert_eq!(22, badges.player_level);

    let badge = badges.badges.get(0).unwrap();
    assert_eq!(1, badge.badgeid);
    assert_eq!(6, badge.level);
    assert_eq!(440, badge.appid);
}

#[test]
fn parse_missing_result() {
    let boxed_parse = parse_response("{}".to_string());
    assert!(boxed_parse.is_err());
}
