use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::isteam_user::get_player_bans::{get_api_url, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url() {
    let api_url = get_api_url(vec![76561197960435530]);

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/ISteamUser/GetPlayerBans/v1", components.path);

    let params = components.query.unwrap();
    let boxed_steamids = params.get("steamids");
    assert_eq!("76561197960435530", boxed_steamids.unwrap().to_string());

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn parse() {
    let response = r#"{
        "players": [
            {
                "SteamId": "76561197960435530",
                "CommunityBanned": false,
                "VACBanned": true,
                "NumberOfVACBans": 1,
                "DaysSinceLastBan": 42,
                "NumberOfGameBans": 0,
                "EconomyBan": "none"
            }
        ]
    }"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let bans = boxed_parse.unwrap();
    assert_eq!(1, bans.len());

    let ban = bans.get(0).unwrap();
    assert_eq!("76561197960435530", ban.steamid);
    assert_eq!(false, ban.community_banned);
    assert_eq!(true, ban.vac_banned);
    assert_eq!(1, ban.number_of_vac_bans);
    assert_eq!(42, ban.days_since_last_ban);
    assert_eq!(0, ban.number_of_game_bans);
    assert_eq!("none", ban.economy_ban);
}

#[test]
fn parse_missing_players() {
    let boxed_parse = parse_response("{}".to_string());
    assert!(boxed_parse.is_err());
}
