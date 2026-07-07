use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::iplayer_service::get_owned_games::{get_api_url, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url() {
    let api_url = get_api_url(76561197960435530, Some(true), Some(true));

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/IPlayerService/GetOwnedGames/v1", components.path);

    let params = components.query.unwrap();
    let boxed_steamid = params.get("steamid");
    assert_eq!(76561197960435530, boxed_steamid.unwrap().parse::<u64>().unwrap());

    let boxed_include_appinfo = params.get("include_appinfo");
    assert_eq!("true", boxed_include_appinfo.unwrap().to_string());

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn parse() {
    let response = r#"{
        "response": {
            "game_count": 1,
            "games": [
                {
                    "appid": 570,
                    "name": "Dota 2",
                    "playtime_forever": 12345,
                    "img_icon_url": "abc123",
                    "has_community_visible_stats": true,
                    "playtime_windows_forever": 100,
                    "playtime_mac_forever": 0,
                    "playtime_linux_forever": 0,
                    "rtime_last_played": 1234567890
                }
            ]
        }
    }"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let owned_games = boxed_parse.unwrap();
    assert_eq!(1, owned_games.game_count);
    assert_eq!(1, owned_games.games.len());

    let game = owned_games.games.get(0).unwrap();
    assert_eq!(570, game.appid);
    assert_eq!("Dota 2", game.name);
    assert_eq!(12345, game.playtime_forever);
    assert_eq!(true, game.has_community_visible_stats);
}

#[test]
fn parse_missing_result() {
    let boxed_parse = parse_response("{}".to_string());
    assert!(boxed_parse.is_err());
}
