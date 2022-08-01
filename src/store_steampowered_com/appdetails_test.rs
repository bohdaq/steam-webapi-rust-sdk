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

#[test]
fn test_make_api_call() {
    let app_id = 147730;
    let result = appdetails::make_api_call(app_id);

    assert!(result.is_ok());

    let response = result.unwrap();
    println!("{}", &response);
    assert!(response.len() > 0);
}

#[test]
fn test_parse_api_call_result() {
    let app_id = 730;
    let result = appdetails::make_api_call(app_id);

    assert!(result.is_ok());

    let response = result.unwrap();
    println!("{}", &response);
    assert!(response.len() > 0);

    let boxed_result = appdetails::parse_api_call_result(response, app_id);
    assert!(boxed_result.is_ok());

    let steam_app = boxed_result.unwrap();
    assert_eq!(steam_app.app_id, app_id);
    assert_eq!(steam_app.name, "Counter-Strike: Global Offensive");
    assert_eq!(steam_app.reviews, "");
    assert_eq!(steam_app.header_image, "https://cdn.akamai.steamstatic.com/steam/apps/730/header.jpg?t=1641233427");
    assert_eq!(steam_app.website, "http://blog.counter-strike.net/");

    let description = "Counter-Strike: Global Offensive (CS: GO) expands upon the team-based action gameplay that it pioneered when it was launched 19 years ago.<br />\r\n<br />\r\nCS: GO features new maps, characters, weapons, and game modes, and delivers updated versions of the classic CS content (de_dust2, etc.).<br />\r\n<br />\r\n&quot;Counter-Strike took the gaming industry by surprise when the unlikely MOD became the most played online PC action game in the world almost immediately after its release in August 1999,&quot; said Doug Lombardi at Valve. &quot;For the past 12 years, it has continued to be one of the most-played games in the world, headline competitive gaming tournaments and selling over 25 million units worldwide across the franchise. CS: GO promises to expand on CS' award-winning gameplay and deliver it to gamers on the PC as well as the next gen consoles and the Mac.&quot;";
    assert_eq!(steam_app.detailed_description, description);
}