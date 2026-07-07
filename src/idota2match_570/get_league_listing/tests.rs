use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::idota2match_570::get_league_listing::{get_api_url, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url() {
    let api_url = get_api_url(None);

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/IDOTA2Match_570/GetLeagueListing/v1", components.path);

    let params = components.query.unwrap();
    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn parse() {
    let response = r#"{
        "result": {
            "leagues": [
                {
                    "name": "The International 2022",
                    "leagueid": 14268,
                    "description": "",
                    "tournament_url": "http://www.dota2.com/international"
                }
            ]
        }
    }"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let leagues = boxed_parse.unwrap();
    assert_eq!(1, leagues.len());

    let league = leagues.get(0).unwrap();
    assert_eq!(14268, league.leagueid);
    assert_eq!("The International 2022", league.name);
}

#[test]
fn parse_missing_leagues() {
    let boxed_parse = parse_response("{}".to_string());
    assert!(boxed_parse.is_err());
}
