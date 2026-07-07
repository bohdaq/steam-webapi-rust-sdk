use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::isteam_user_stats::get_user_stats_for_game::{get_api_url, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url() {
    let api_url = get_api_url(76561197960435530, 440);

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/ISteamUserStats/GetUserStatsForGame/v2", components.path);

    let params = components.query.unwrap();
    let boxed_steamid = params.get("steamid");
    assert_eq!(76561197960435530, boxed_steamid.unwrap().parse::<u64>().unwrap());

    let boxed_appid = params.get("appid");
    assert_eq!(440, boxed_appid.unwrap().parse::<i64>().unwrap());

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn parse() {
    let response = r#"{
        "playerstats": {
            "steamID": "76561197960435530",
            "gameName": "Team Fortress 2",
            "stats": [
                {"name": "Kills", "value": 1234}
            ],
            "achievements": [
                {"name": "TF_SCOUT_LONG_DISTANCE_RUNNER", "achieved": 1}
            ]
        }
    }"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let user_stats = boxed_parse.unwrap();
    assert_eq!("76561197960435530", user_stats.steamid);
    assert_eq!("Team Fortress 2", user_stats.game_name);
    assert_eq!(1, user_stats.stats.len());
    assert_eq!("Kills", user_stats.stats.get(0).unwrap().name);
    assert_eq!(1234.0, user_stats.stats.get(0).unwrap().value);

    assert_eq!(1, user_stats.achievements.len());
    let achievement = user_stats.achievements.get(0).unwrap();
    assert_eq!("TF_SCOUT_LONG_DISTANCE_RUNNER", achievement.name);
    assert_eq!(true, achievement.achieved);
}

#[test]
fn parse_missing_playerstats() {
    let boxed_parse = parse_response("{}".to_string());
    assert!(boxed_parse.is_err());
}
