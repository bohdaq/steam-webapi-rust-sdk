use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::isteam_user_stats::get_schema_for_game::{get_api_url, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url() {
    let api_url = get_api_url(440);

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/ISteamUserStats/GetSchemaForGame/v2", components.path);

    let params = components.query.unwrap();
    let boxed_appid = params.get("appid");
    assert_eq!(440, boxed_appid.unwrap().parse::<i64>().unwrap());

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn parse() {
    let response = r#"{
        "game": {
            "gameName": "Team Fortress 2",
            "gameVersion": "3",
            "availableGameStats": {
                "stats": [
                    {"name": "Kills", "defaultvalue": 0, "displayName": "Kills"}
                ],
                "achievements": [
                    {
                        "name": "TF_SCOUT_LONG_DISTANCE_RUNNER",
                        "defaultvalue": 0,
                        "displayName": "Long Distance Runner",
                        "hidden": 0,
                        "description": "Travel 10 miles as the Scout in a single life.",
                        "icon": "icon.jpg",
                        "icongray": "icon_gray.jpg"
                    }
                ]
            }
        }
    }"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let schema = boxed_parse.unwrap();
    assert_eq!("Team Fortress 2", schema.game_name);
    assert_eq!("3", schema.game_version);
    assert_eq!(1, schema.stats.len());
    assert_eq!("Kills", schema.stats.get(0).unwrap().name);

    assert_eq!(1, schema.achievements.len());
    let achievement = schema.achievements.get(0).unwrap();
    assert_eq!("TF_SCOUT_LONG_DISTANCE_RUNNER", achievement.name);
    assert_eq!("Long Distance Runner", achievement.display_name);
    assert_eq!(false, achievement.hidden);
}

#[test]
fn parse_missing_game() {
    let boxed_parse = parse_response("{}".to_string());
    assert!(boxed_parse.is_err());
}
