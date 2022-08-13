use crate::{get_app_details, get_app_list, get_cached_app_details, get_cached_app_list};

#[cfg(test)]

#[test]
fn test_get_app_details() {
    let app_id = 570;
    let app = get_app_details(app_id).unwrap();

    assert_eq!(app.name, "Dota 2".to_string());
}

#[test]
fn test_get_cached_app_details() {
    let app_id = 730;
    let app = get_cached_app_details(app_id).unwrap();

    assert_eq!(app.name, "Counter-Strike: Global Offensive".to_string());
}

#[test]
fn test_get_app_list() {
    let boxed_steam_app_list = get_app_list();
    assert!(boxed_steam_app_list.is_ok());

    let steam_app_list = boxed_steam_app_list.unwrap();
    assert!(steam_app_list.len()>0);
    let steam_app = steam_app_list.get(0).unwrap();
    assert!(steam_app.appid > 0);

    assert!(steam_app.name.len() > 0);
}

#[test]
fn test_get_cached_app_list() {
    let boxed_steam_app_list = get_cached_app_list();
    assert!(boxed_steam_app_list.is_ok());


    let steam_app_list = boxed_steam_app_list.unwrap();

    assert!(steam_app_list.len()>0);
    let steam_app = steam_app_list.get(0).unwrap();
    assert!(steam_app.appid > 0);

    assert!(steam_app.name.len() > 0);
}

#[test]
fn test_details_on_cached_resources() {
    let boxed_steam_app_list = get_cached_app_list();
    assert!(boxed_steam_app_list.is_ok());

    let steam_app_list = boxed_steam_app_list.unwrap();
    for steam_app in steam_app_list {
        let app_id = steam_app.appid;

        let boxed_result = get_cached_app_details(app_id);
        if boxed_result.is_ok() {
            let app_details = boxed_result.unwrap();
            println!("result is ok for {} app id {}", app_details.name, app_details.app_id);
            
        } else {
            let error_message = boxed_result.err().unwrap();
            println!("{} {}", error_message, app_id);

        };
    }
}