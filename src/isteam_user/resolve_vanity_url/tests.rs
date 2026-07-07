use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::isteam_user::resolve_vanity_url::{get_api_url, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url() {
    let api_url = get_api_url("gabelogannewell".to_string(), Some(1));

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/ISteamUser/ResolveVanityURL/v1", components.path);

    let params = components.query.unwrap();
    let boxed_vanityurl = params.get("vanityurl");
    assert_eq!("gabelogannewell", boxed_vanityurl.unwrap().to_string());

    let boxed_url_type = params.get("url_type");
    assert_eq!("1", boxed_url_type.unwrap().to_string());

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn parse_success() {
    let response = r#"{"response":{"steamid":"76561197960287930","success":1}}"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let resolution = boxed_parse.unwrap();
    assert_eq!(1, resolution.success);
    assert_eq!("76561197960287930", resolution.steamid);
    assert_eq!("", resolution.message);
}

#[test]
fn parse_no_match() {
    let response = r#"{"response":{"success":42,"message":"No match"}}"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let resolution = boxed_parse.unwrap();
    assert_eq!(42, resolution.success);
    assert_eq!("", resolution.steamid);
    assert_eq!("No match", resolution.message);
}
