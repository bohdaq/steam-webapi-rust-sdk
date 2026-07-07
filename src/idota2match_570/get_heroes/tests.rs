use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::idota2match_570::get_heroes::{get_api_url, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url() {
    let api_url = get_api_url(Some("en".to_string()));

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/IDOTA2Match_570/GetHeroes/v1", components.path);

    let params = components.query.unwrap();
    let boxed_language = params.get("language");
    assert_eq!("en", boxed_language.unwrap().to_string());

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn parse() {
    let response = r#"{
        "result": {
            "heroes": [
                {"name": "npc_dota_hero_antimage", "id": 1},
                {"name": "npc_dota_hero_axe", "id": 2}
            ],
            "count": 2,
            "status": 200
        }
    }"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let heroes = boxed_parse.unwrap();
    assert_eq!(2, heroes.len());
    assert_eq!(1, heroes.get(0).unwrap().id);
    assert_eq!("npc_dota_hero_antimage", heroes.get(0).unwrap().name);
}

#[test]
fn parse_missing_heroes() {
    let boxed_parse = parse_response("{}".to_string());
    assert!(boxed_parse.is_err());
}
