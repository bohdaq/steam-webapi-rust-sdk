use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::isteam_user::get_player_summaries::{get_api_url, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url() {
    let api_url = get_api_url(vec![76561197960435530, 76561197960435531]);

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/ISteamUser/GetPlayerSummaries/v2", components.path);

    let params = components.query.unwrap();
    let boxed_steamids = params.get("steamids");
    assert_eq!("76561197960435530,76561197960435531", boxed_steamids.unwrap().to_string());

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn parse() {
    let response = r#"{
        "response": {
            "players": [
                {
                    "steamid": "76561197960435530",
                    "communityvisibilitystate": 3,
                    "profilestate": 1,
                    "personaname": "Robin",
                    "commentpermission": 1,
                    "profileurl": "https://steamcommunity.com/id/gabelogannewell/",
                    "avatar": "https://avatars.steamstatic.com/a.jpg",
                    "avatarmedium": "https://avatars.steamstatic.com/b.jpg",
                    "avatarfull": "https://avatars.steamstatic.com/c.jpg",
                    "lastlogoff": 1234567890,
                    "personastate": 0,
                    "realname": "Robin Walker",
                    "primaryclanid": "103582791429521412",
                    "timecreated": 1063407589,
                    "loccountrycode": "US",
                    "locstatecode": "WA"
                }
            ]
        }
    }"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let players = boxed_parse.unwrap();
    assert_eq!(1, players.len());

    let player = players.get(0).unwrap();
    assert_eq!("76561197960435530", player.steamid);
    assert_eq!(3, player.communityvisibilitystate);
    assert_eq!("Robin", player.personaname);
    assert_eq!(true, player.comment_permission);
    assert_eq!("US", player.loccountrycode);
    assert_eq!("", player.gameid);
}

#[test]
fn parse_missing_players() {
    let boxed_parse = parse_response("{}".to_string());
    assert!(boxed_parse.is_err());
}
