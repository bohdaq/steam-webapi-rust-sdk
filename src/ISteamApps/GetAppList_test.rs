use crate::ISteamApps::GetAppList;
use crate::ISteamApps::GetAppList::make_api_call;
use crate::ISteamApps::GetAppList::parse_api_call_result;
use crate::util::get_steam_web_api_key;

#[test]
fn test_make_api_call() {
    let response = make_api_call();
    assert!(response.len()>0);
}

#[test]
fn test_parse_api_call_result() {
    let response = make_api_call();
    let steam_app_list = parse_api_call_result(response);

    assert!(steam_app_list.len()>0);
    let steam_app = steam_app_list.get(0).unwrap();
    assert!(steam_app.appid > 0);
    println!("{}", steam_app.name);

    assert!(steam_app.name.len() > 0);

}

#[test]
fn test_get() {
    let steam_app_list = GetAppList::get();

    assert!(steam_app_list.len()>0);
    let steam_app = steam_app_list.get(0).unwrap();
    assert!(steam_app.appid > 0);
    println!("{}", steam_app.name);

    assert!(steam_app.name.len() > 0);
}

#[test]
fn test_get_cached() {
    let steam_app_list = GetAppList::get_cached();

    assert!(steam_app_list.len()>0);
    let steam_app = steam_app_list.get(0).unwrap();
    assert!(steam_app.appid > 0);
    println!("{}", steam_app.name);

    assert!(steam_app.name.len() > 0);
}