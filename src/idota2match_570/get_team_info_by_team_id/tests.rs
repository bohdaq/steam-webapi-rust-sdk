use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::idota2match_570::get_team_info_by_team_id::{get_api_url, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url() {
    let api_url = get_api_url(Some(1), Some(1));

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/IDOTA2Match_570/GetTeamInfoByTeamID/v1", components.path);

    let params = components.query.unwrap();
    let boxed_start_at_team_id = params.get("start_at_team_id");
    assert_eq!(1, boxed_start_at_team_id.unwrap().parse::<u64>().unwrap());

    let boxed_teams_requested = params.get("teams_requested");
    assert_eq!("1", boxed_teams_requested.unwrap().to_string());

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn parse() {
    let response = r#"{
        "result": {
            "teams": [
                {
                    "team_id": 1,
                    "name": "Test Team",
                    "tag": "TT",
                    "time_created": 1234567890,
                    "logo": 999,
                    "logo_sponsor": 0,
                    "country_code": "US",
                    "url": ""
                }
            ],
            "status": 200
        }
    }"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let teams = boxed_parse.unwrap();
    assert_eq!(1, teams.len());

    let team = teams.get(0).unwrap();
    assert_eq!(1, team.team_id);
    assert_eq!("Test Team", team.name);
    assert_eq!("TT", team.tag);
}

#[test]
fn parse_missing_teams() {
    let boxed_parse = parse_response("{}".to_string());
    assert!(boxed_parse.is_err());
}
