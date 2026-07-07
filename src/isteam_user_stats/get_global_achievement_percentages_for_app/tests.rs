use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::isteam_user_stats::get_global_achievement_percentages_for_app::{get_api_url, parse_response};

#[test]
fn api_url() {
    let api_url = get_api_url(440);

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/ISteamUserStats/GetGlobalAchievementPercentagesForApp/v2", components.path);

    let params = components.query.unwrap();
    let boxed_gameid = params.get("gameid");
    assert_eq!(440, boxed_gameid.unwrap().parse::<i64>().unwrap());
}

#[test]
fn parse() {
    let response = r#"{
        "achievementpercentages": {
            "achievements": [
                {"name": "TF_SCOUT_LONG_DISTANCE_RUNNER", "percent": 65.099998}
            ]
        }
    }"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let achievements = boxed_parse.unwrap();
    assert_eq!(1, achievements.len());

    let achievement = achievements.get(0).unwrap();
    assert_eq!("TF_SCOUT_LONG_DISTANCE_RUNNER", achievement.name);
    assert_eq!(65.099998, achievement.percent);
}

#[test]
fn parse_missing_achievements() {
    let boxed_parse = parse_response("{}".to_string());
    assert!(boxed_parse.is_err());
}
