use crate::isteam_apps::get_app_list::make_api_call;
use crate::isteam_apps::get_app_list::parse_api_call_result;
use crate::isteam_apps::get_app_list::get;
use crate::isteam_apps::get_app_list::get_cached;
use crate::isteam_apps::get_app_list::get_resource_filepath;
use std::fs::metadata;

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
    let mut cache_timestamp = 0;
    let boxed_metadata = metadata(get_resource_filepath());
    if boxed_metadata.is_ok() {
        cache_timestamp = boxed_metadata.unwrap().modified().unwrap().elapsed().unwrap().as_millis();
    }

    let steam_app_list = get();

    assert!(steam_app_list.len()>0);
    let steam_app = steam_app_list.get(0).unwrap();
    assert!(steam_app.appid > 0);

    assert!(steam_app.name.len() > 0);

    let mut latest_cache_timestamp = 0;
    let boxed_metadata = metadata(get_resource_filepath());
    if boxed_metadata.is_ok() {
        latest_cache_timestamp = boxed_metadata.unwrap().modified().unwrap().elapsed().unwrap().as_millis();
    }

    let cache_is_updated = latest_cache_timestamp < cache_timestamp;
    assert!(cache_is_updated);
}

#[test]
fn test_get_cached() {
    let steam_app_list = get_cached();

    assert!(steam_app_list.len()>0);
    let steam_app = steam_app_list.get(0).unwrap();
    assert!(steam_app.appid > 0);

    assert!(steam_app.name.len() > 0);
}