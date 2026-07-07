use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::iplayer_service::get_recently_played_games::{get_api_url, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url() {
    let api_url = get_api_url(76561197960435530, Some(3));

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/IPlayerService/GetRecentlyPlayedGames/v1", components.path);

    let params = components.query.unwrap();
    let boxed_steamid = params.get("steamid");
    assert_eq!(76561197960435530, boxed_steamid.unwrap().parse::<u64>().unwrap());

    let boxed_count = params.get("count");
    assert_eq!("3", boxed_count.unwrap().to_string());

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn parse() {
    let response = r#"{
        "response": {
            "total_count": 1,
            "games": [
                {
                    "appid": 570,
                    "name": "Dota 2",
                    "playtime_2weeks": 120,
                    "playtime_forever": 12345,
                    "img_icon_url": "abc123"
                }
            ]
        }
    }"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let recently_played = boxed_parse.unwrap();
    assert_eq!(1, recently_played.total_count);
    assert_eq!(1, recently_played.games.len());

    let game = recently_played.games.get(0).unwrap();
    assert_eq!(570, game.appid);
    assert_eq!("Dota 2", game.name);
    assert_eq!(120, game.playtime_2weeks);
    assert_eq!(12345, game.playtime_forever);
}

#[test]
fn parse_no_recent_games() {
    let response = r#"{"response": {}}"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let recently_played = boxed_parse.unwrap();
    assert_eq!(0, recently_played.total_count);
    assert_eq!(0, recently_played.games.len());
}
