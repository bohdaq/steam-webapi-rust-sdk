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
fn test_get_cache_dir_path() {
    let app_id = 730;
    let cache_dir_path = appdetails::get_cache_dir_path(app_id);

    assert_eq!(cache_dir_path, "steam-webapi-cache/steampowered/appdetails/0/730/".to_string());
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

#[test]
fn test_get_api_url() {
    let app_id = 147730;
    let api_url = appdetails::get_api_url(app_id);

    let expected_api_url = format!("https://store.steampowered.com/api/appdetails?appids={}&lang=en", app_id);

    assert_eq!(api_url, expected_api_url.to_string());
}