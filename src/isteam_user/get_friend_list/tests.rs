use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::isteam_user::get_friend_list::{get_api_url, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url_default_relationship() {
    let api_url = get_api_url(76561197960435530, None);

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/ISteamUser/GetFriendList/v1", components.path);

    let params = components.query.unwrap();
    let boxed_steamid = params.get("steamid");
    assert_eq!(76561197960435530, boxed_steamid.unwrap().parse::<u64>().unwrap());

    let boxed_relationship = params.get("relationship");
    assert_eq!("friend", boxed_relationship.unwrap().to_string());

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn parse() {
    let response = r#"{
        "friendslist": {
            "friends": [
                {
                    "steamid": "76561197960265731",
                    "relationship": "friend",
                    "friend_since": 1300856234
                }
            ]
        }
    }"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let friends = boxed_parse.unwrap();
    assert_eq!(1, friends.len());

    let friend = friends.get(0).unwrap();
    assert_eq!("76561197960265731", friend.steamid);
    assert_eq!("friend", friend.relationship);
    assert_eq!(1300856234, friend.friend_since);
}

#[test]
fn parse_private_friends_list() {
    let boxed_parse = parse_response("{}".to_string());
    assert!(boxed_parse.is_err());
}
