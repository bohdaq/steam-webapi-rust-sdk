use crate::store_steampowered_com::appdetails;

#[test]
fn test_get() {
    let app_id = 570;
    let app = appdetails::get(app_id).unwrap();

    assert_eq!(app.name, "Dota 2".to_string());
}

#[test]
fn test_get_cached() {
    let app_id = 730;
    let app = appdetails::get_cached(app_id).unwrap();

    assert_eq!(app.name, "Counter-Strike: Global Offensive".to_string());
}

#[test]
fn test_get_resource_filepath() {
    let app_id = 730;
    let resource_filepath = appdetails::get_resource_filepath(app_id);

    assert_eq!(resource_filepath, "steam-webapi-cache/steampowered/appdetails/0/730/730.json".to_string());
}

#[test]
fn test_get_resource_filepath_long_number() {
    let app_id = 147730;
    let resource_filepath = appdetails::get_resource_filepath(app_id);

    assert_eq!(resource_filepath, "steam-webapi-cache/steampowered/appdetails/14/147730/147730.json".to_string());
}