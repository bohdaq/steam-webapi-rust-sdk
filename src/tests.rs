use crate::{convert_32bit_account_id_to_64bit, convert_64bit_account_id_to_32bit, get_app_details, get_app_list, get_cached_app_details, get_cached_app_list};

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


#[test]
fn test_steam_32bit_to_64bit_id_conversion() {
    let _32bit_id = 95816;

    let converted = convert_32bit_account_id_to_64bit(_32bit_id);

    let expected_id = 76561197960361544;
    assert_eq!(expected_id, converted);
}

#[test]
fn test_steam_64bit_to_32bit_id_conversion() {
    let _64bit_id = 76561199148118849;

    let converted = convert_64bit_account_id_to_32bit(_64bit_id);

    let expected_id = 1187853121;
    assert_eq!(expected_id, converted);
}

#[test]
fn test_steam_64bit_to_32bit_id_conversion_v2() {
    let _64bit_id = 76561197998367327;

    let converted = convert_64bit_account_id_to_32bit(_64bit_id);

    let expected_id = 38101599;
    assert_eq!(expected_id, converted);
}