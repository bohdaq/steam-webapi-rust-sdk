use crate::isteam_apps::get_app_list::make_api_call;
use crate::isteam_apps::get_app_list::parse_api_call_result;
use crate::isteam_apps::get_app_list::get;
use crate::isteam_apps::get_app_list::get_cached;

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

    assert!(steam_app.name.len() > 0);

}

#[test]
fn test_get() {
    let steam_app_list = get();

    assert!(steam_app_list.len()>0);
    let steam_app = steam_app_list.get(0).unwrap();
    assert!(steam_app.appid > 0);

    assert!(steam_app.name.len() > 0);
}

#[test]
fn test_get_cached() {
    let steam_app_list = get_cached();

    assert!(steam_app_list.len()>0);
    let steam_app = steam_app_list.get(0).unwrap();
    assert!(steam_app.appid > 0);

    assert!(steam_app.name.len() > 0);
}