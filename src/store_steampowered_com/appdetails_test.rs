use crate::store_steampowered_com;
#[cfg(test)]

#[test]
fn test_get() {
    let app_id = 570;
    let app = store_steampowered_com::appdetails::get(app_id).unwrap();

    assert_eq!(app.name, "Dota 2".to_string());
    assert_eq!(app.reviews, "“A modern multiplayer masterpiece.”<br>9.5/10 – <a href=\"https://www.destructoid.com/review-dota-2-258506.phtml\" target=\"_blank\" rel=\"noreferrer\"  >Destructoid</a><br><br>“Once you start to learn its secrets, there’s a wild and exciting variety of play here that’s unmatched, even by its peers.”<br>9.4/10 – <a href=\"http://www.ign.com/articles/2013/07/24/dota-2-review\" target=\"_blank\" rel=\"noreferrer\"  >IGN</a><br><br>“Dota 2 is possibly the only competitive free-to-play game that is totally uncompromised by its business model.”<br>90/100 – <a href=\"http://www.pcgamer.com/dota-2-review-2/\" target=\"_blank\" rel=\"noreferrer\"  >PC Gamer</a><br>".to_string());
}

#[test]
fn test_get_2210() {
    let app_id = 2210;
    let app = store_steampowered_com::appdetails::get(app_id).unwrap();

    assert_eq!(app.required_age, 18);
}

#[test]
fn test_get_1070410() {
    let app_id = 1070410;
    let app = store_steampowered_com::appdetails::get(app_id).unwrap();

    assert_eq!(app.price_overview.recurring_sub_desc, "279₴ at checkout, auto-renewed every 1 month(s) at 169₴.");
    assert_eq!(app.price_overview.recurring_sub, 414304);
    assert_eq!(app.price_overview.currency, "UAH");
    assert_eq!(app.price_overview.final_price, 29900);
    assert_eq!(app.price_overview.discount_percent, 0);
    assert_eq!(app.price_overview.initial, 29900);
    assert_eq!(app.price_overview.initial_formatted, "");
}

#[test]
fn test_get_1900() {
    let app_id = 1900;
    let app = store_steampowered_com::appdetails::get(app_id).unwrap();

    assert_eq!(app.price_overview.initial_formatted, "");
}

#[test]
fn test_get_550() {
    let app_id = 550;
    let app = store_steampowered_com::appdetails::get(app_id).unwrap();

    assert_eq!(app.pc_requirements.recommended, "<strong>Recommended:</strong><br><ul class=\"bb_ul\"><li><strong>OS:</strong> Windows® 7 32/64-bit / Vista 32/64 / XP<br></li><li><strong>Processor:</strong> Intel core 2 duo 2.4GHz<br></li><li><strong>Memory:</strong> 2 GB RAM<br></li><li><strong>Graphics:</strong> Video Card Shader model 3.0. NVidia 7600, ATI X1600 or better<br></li><li><strong>DirectX:</strong> Version 9.0c<br></li><li><strong>Storage:</strong> 13 GB available space<br></li><li><strong>Sound Card:</strong> DirectX 9.0c compatible sound card</li></ul>");
    assert_eq!(app.pc_requirements.minimum, "<strong>Minimum:</strong><br><ul class=\"bb_ul\"><li><strong>OS:</strong> Windows® 7 32/64-bit / Vista 32/64 / XP<br></li><li><strong>Processor:</strong> Pentium 4 3.0GHz<br></li><li><strong>Memory:</strong> 2 GB RAM<br></li><li><strong>Graphics:</strong> Video card with 128 MB, Shader model 2.0. ATI X800, NVidia 6600 or better<br></li><li><strong>DirectX:</strong> Version 9.0c<br></li><li><strong>Storage:</strong> 13 GB available space<br></li><li><strong>Sound Card:</strong> DirectX 9.0c compatible sound card</li></ul>");
}

#[test]
fn test_get_320() {
    let app_id = 320;
    let app = store_steampowered_com::appdetails::get(app_id).unwrap();

    assert_eq!(app.mac_requirements.recommended, "");
    assert_eq!(app.mac_requirements.minimum, "<strong>Minimum: </strong>OS X version Leopard 10.5.8, Snow Leopard 10.6.3, 1GB RAM, NVIDIA GeForce 8 or higher, ATI X1600 or higher, or Intel HD 3000 or higher Mouse, Keyboard, Internet Connection");
}

#[test]
fn test_get_cached_730() {
    let app_id = 730;
    let app = store_steampowered_com::appdetails::get_cached(app_id).unwrap();

    assert_eq!(app.name, "Counter-Strike: Global Offensive".to_string());
    assert_eq!(app.app_type, "game".to_string());
    assert_eq!(app.supported_languages, "Czech, Danish, Dutch, English<strong>*</strong>, Finnish, French, German, Hungarian, Italian, Japanese, Korean, Norwegian, Polish, Portuguese, Portuguese - Brazil, Romanian, Russian, Simplified Chinese, Spanish - Spain, Swedish, Thai, Traditional Chinese, Turkish, Bulgarian, Ukrainian, Greek, Spanish - Latin America, Vietnamese<br><strong>*</strong>languages with full audio support".to_string());
    assert_eq!(app.support_info.url, "".to_string());
    assert_eq!(app.support_info.email, "".to_string());
    assert_eq!(app.short_description, "Counter-Strike: Global Offensive (CS: GO) expands upon the team-based action gameplay that it pioneered when it was launched 19 years ago. CS: GO features new maps, characters, weapons, and game modes, and delivers updated versions of the classic CS content (de_dust2, etc.).".to_string());

    let first_screen = app.screenshots.get(0).unwrap();
    assert_eq!(first_screen.id, 0);
    assert_eq!(first_screen.path_thumbnail, "https://cdn.akamai.steamstatic.com/steam/apps/730/ss_118cb022b9a43f70d2e5a2df7427f29088b6b191.600x338.jpg?t=1641233427");
    assert_eq!(first_screen.path_full, "https://cdn.akamai.steamstatic.com/steam/apps/730/ss_118cb022b9a43f70d2e5a2df7427f29088b6b191.1920x1080.jpg?t=1641233427");

    let last_screen = app.screenshots.get(10).unwrap();
    assert_eq!(last_screen.id, 10);
    assert_eq!(last_screen.path_thumbnail, "https://cdn.akamai.steamstatic.com/steam/apps/730/ss_60b4f959497899515f46012df805b0006ef21af6.600x338.jpg?t=1641233427");
    assert_eq!(last_screen.path_full, "https://cdn.akamai.steamstatic.com/steam/apps/730/ss_60b4f959497899515f46012df805b0006ef21af6.1920x1080.jpg?t=1641233427");

    assert_eq!(app.required_age, 0);
    assert_eq!(app.release_date.coming_soon, false);
    assert_eq!(app.release_date.date, "Aug 21, 2012");
    assert_eq!(app.recommendations.total, 3452092);
    assert_eq!(app.platforms.windows, true);
    assert_eq!(app.platforms.mac, true);
    assert_eq!(app.platforms.linux, true);

    assert_eq!(app.pc_requirements.recommended, "");
    assert_eq!(app.pc_requirements.minimum, "<strong>Minimum:</strong><br><ul class=\"bb_ul\"><li><strong>OS:</strong> Windows® 7/Vista/XP<br></li><li><strong>Processor:</strong> Intel® Core™ 2 Duo E6600 or AMD Phenom™ X3 8750 processor or better<br></li><li><strong>Memory:</strong> 2 GB RAM<br></li><li><strong>Graphics:</strong> Video card must be 256 MB or more and should be a DirectX 9-compatible with support for Pixel Shader 3.0<br></li><li><strong>DirectX:</strong> Version 9.0c<br></li><li><strong>Storage:</strong> 15 GB available space</li></ul>");

    assert_eq!(app.mac_requirements.recommended, "");
    assert_eq!(app.mac_requirements.minimum, "<strong>Minimum:</strong><br><ul class=\"bb_ul\"><li><strong>OS:</strong> MacOS X 10.11 (El Capitan) or later<br></li><li><strong>Processor:</strong> Intel Core Duo Processor (2GHz or better)<br></li><li><strong>Memory:</strong> 2 GB RAM<br></li><li><strong>Graphics:</strong> ATI Radeon HD 2400 or better / NVIDIA 8600M or better<br></li><li><strong>Storage:</strong> 15 GB available space</li></ul>");
    assert_eq!(app.package_groups.get(0).unwrap().title, "Buy Counter-Strike: Global Offensive");
    assert_eq!(app.package_groups.get(0).unwrap().selection_text, "Select a purchase option");
    assert_eq!(app.package_groups.get(0).unwrap().name, "default");
    assert_eq!(app.package_groups.get(0).unwrap().save_text, "");
    assert_eq!(app.package_groups.get(0).unwrap().is_recurring_subscription, "false");
    assert_eq!(app.package_groups.get(0).unwrap().display_type, "0");
    assert_eq!(app.package_groups.get(0).unwrap().subs.get(0).unwrap().packageid, 298963);
    assert_eq!(app.package_groups.get(0).unwrap().subs.get(0).unwrap().price_in_cents_with_discount, 0);

    assert_eq!(app.package_groups.get(0).unwrap().subs.len(), 2);

    assert_eq!(app.package_groups.get(0).unwrap().subs.get(1).unwrap().packageid, 54029);
    assert_eq!(app.package_groups.get(0).unwrap().subs.get(1).unwrap().price_in_cents_with_discount, 1499);

    assert_eq!(app.movies.get(0).unwrap().thumbnail, "https://cdn.akamai.steamstatic.com/steam/apps/81958/movie.293x165.jpg?t=1554409259");
    assert_eq!(app.movies.get(0).unwrap().name, "CS:GO Trailer Long");
    assert_eq!(app.movies.get(0).unwrap().id, 81958);
    assert_eq!(app.movies.get(0).unwrap().highlight, true);
    assert_eq!(app.movies.len(), 8);

}

#[test]
fn test_get_cached_214150() {
    let app_id = 214150;
    let app = store_steampowered_com::appdetails::get_cached(app_id).unwrap();
    assert_eq!(app.package_groups.get(0).unwrap().display_type, "default");
    assert_eq!(app.package_groups.get(0).unwrap().description, "");
}

#[test]
fn test_get_cache_dir_path() {
    let app_id = 730;
    let cache_dir_path = store_steampowered_com::appdetails::get_cache_dir_path(app_id);

    assert_eq!(cache_dir_path, "steam-webapi-cache/steampowered/appdetails/0/730/".to_string());
}


#[test]
fn test_get_resource_filepath() {
    let app_id = 730;
    let resource_filepath = store_steampowered_com::appdetails::get_resource_filepath(app_id);

    assert_eq!(resource_filepath, "steam-webapi-cache/steampowered/appdetails/0/730/730.json".to_string());
}

#[test]
fn test_get_resource_filepath_long_number() {
    let app_id = 147730;
    let resource_filepath = store_steampowered_com::appdetails::get_resource_filepath(app_id);

    assert_eq!(resource_filepath, "steam-webapi-cache/steampowered/appdetails/14/147730/147730.json".to_string());
}

#[test]
fn test_get_api_url() {
    let app_id = 147730;
    let api_url = store_steampowered_com::appdetails::get_api_url(app_id);

    let expected_api_url = format!("https://store.steampowered.com/api/appdetails?appids={}&lang=en", app_id);

    assert_eq!(api_url, expected_api_url.to_string());
}

#[test]
fn test_make_api_call() {
    let app_id = 147730;
    let result = store_steampowered_com::appdetails::make_api_call(app_id);

    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response.len() > 0);
}

#[test]
fn test_parse_api_call_result() {
    let app_id = 730;
    let result = store_steampowered_com::appdetails::make_api_call(app_id);

    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response.len() > 0);

    let boxed_result = store_steampowered_com::appdetails::parse_api_call_result(response, app_id);
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