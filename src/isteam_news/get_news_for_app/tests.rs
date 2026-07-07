use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::isteam_news::get_news_for_app::{get_api_url, parse_response};

#[test]
fn api_url() {
    let api_url = get_api_url(440, Some(3), Some(300));

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/ISteamNews/GetNewsForApp/v2", components.path);

    let params = components.query.unwrap();
    let boxed_appid = params.get("appid");
    assert_eq!(440, boxed_appid.unwrap().parse::<i64>().unwrap());

    let boxed_count = params.get("count");
    assert_eq!("3", boxed_count.unwrap().to_string());

    let boxed_maxlength = params.get("maxlength");
    assert_eq!("300", boxed_maxlength.unwrap().to_string());
}

#[test]
fn parse() {
    let response = r#"{
        "appnews": {
            "appid": 440,
            "newsitems": [
                {
                    "gid": "1234567890123456789",
                    "title": "Team Fortress 2 Update Released",
                    "url": "http://store.steampowered.com/news/",
                    "is_external_url": true,
                    "author": "Valve",
                    "contents": "An update has been released.",
                    "feedlabel": "Team Fortress 2 Updates",
                    "date": 1234567890,
                    "feedname": "TF2_facebook"
                }
            ],
            "count": 397
        }
    }"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let newsitems = boxed_parse.unwrap();
    assert_eq!(1, newsitems.len());

    let item = newsitems.get(0).unwrap();
    assert_eq!("1234567890123456789", item.gid);
    assert_eq!("Team Fortress 2 Update Released", item.title);
    assert_eq!(true, item.is_external_url);
    assert_eq!("Valve", item.author);
}

#[test]
fn parse_missing_newsitems() {
    let boxed_parse = parse_response("{}".to_string());
    assert!(boxed_parse.is_err());
}
