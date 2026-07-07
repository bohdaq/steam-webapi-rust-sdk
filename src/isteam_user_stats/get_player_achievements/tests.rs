use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::isteam_user_stats::get_player_achievements::{get_api_url, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url() {
    let api_url = get_api_url(76561197960435530, 440, Some("english".to_string()));

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/ISteamUserStats/GetPlayerAchievements/v1", components.path);

    let params = components.query.unwrap();
    let boxed_steamid = params.get("steamid");
    assert_eq!(76561197960435530, boxed_steamid.unwrap().parse::<u64>().unwrap());

    let boxed_appid = params.get("appid");
    assert_eq!(440, boxed_appid.unwrap().parse::<i64>().unwrap());

    let boxed_l = params.get("l");
    assert_eq!("english", boxed_l.unwrap().to_string());

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn parse() {
    let response = r#"{
        "playerstats": {
            "steamID": "76561197960435530",
            "gameName": "Team Fortress 2",
            "achievements": [
                {
                    "apiname": "TF_PLAY_GAME_EVERYMAP",
                    "achieved": 1,
                    "unlocktime": 1234567890,
                    "name": "Grand Tourist",
                    "description": "Play a complete round on every official map."
                }
            ],
            "success": true
        }
    }"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let player_achievements = boxed_parse.unwrap();
    assert_eq!("76561197960435530", player_achievements.steamid);
    assert_eq!("Team Fortress 2", player_achievements.game_name);
    assert_eq!(1, player_achievements.achievements.len());

    let achievement = player_achievements.achievements.get(0).unwrap();
    assert_eq!("TF_PLAY_GAME_EVERYMAP", achievement.apiname);
    assert_eq!(true, achievement.achieved);
    assert_eq!(1234567890, achievement.unlocktime);
    assert_eq!("Grand Tourist", achievement.name);
}

#[test]
fn parse_private_profile() {
    let response = r#"{"playerstats": {"error": "Profile is not public", "success": false}}"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_err());
    assert_eq!("Profile is not public", boxed_parse.err().unwrap());
}
