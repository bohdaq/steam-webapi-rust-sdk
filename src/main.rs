// curl https://api.steampowered.com/IDOTA2Match_570/GetMatchHistory/V001/\?key\=1F2709FC907F0DEE1D1EB4787E06B695\&account_id\=1187853121
// curl https://api.steampowered.com/IDOTA2Match_570/GetMatchDetails/V001/\?match_id\=6644665007\&key\=1F2709FC907F0DEE1D1EB4787E06B695
// curl https://api.steampowered.com/IEconDOTA2_205790/GetHeroes/V001/\?key\=1F2709FC907F0DEE1D1EB4787E06B695
// curl https://api.steampowered.com/IEconDOTA2_205790/GetGameItems/V001/\?key\=1F2709FC907F0DEE1D1EB4787E06B695
// curl https://api.steampowered.com/IEconDOTA2_205790/GetGameItems/V001/\?key\=1F2709FC907F0DEE1D1EB4787E06B695
// curl https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/\?key\=1F2709FC907F0DEE1D1EB4787E06B695\&steamids\=76561199148118849,76561197998367327
// http://whatsmysteamid.azurewebsites.net/
// https://store.steampowered.com/api/appdetails?appids=730&lang=ua
// https://cdn.akamai.steamstatic.com/steam/apps/730/capsule_231x87.jpg
// https://cdn.dota2.com/apps/dota2/images/items/super_blink_lg.png
// https://cdn.dota2.com/apps/dota2/images/heroes/dark_willow_full.png


use std::{thread, time};

pub mod util;
pub mod isteam_apps;
pub mod store_steampowered_com;

fn main() {
    println!("Steam Web API Rust SDK");

    let app_list = isteam_apps::get_app_list::get_cached();
    for app in app_list {
        get_app_details(app.appid)
    }
}

fn get_app_details(app_id: i64) {
    println!("\n\n");
    let boxed_result = store_steampowered_com::appdetails::get_cached(app_id);
    if boxed_result.is_ok() {
        let app_details = boxed_result.unwrap();
        println!("result is ok for {} app id {}", app_details.name, app_details.app_id);

    } else {
        let error_message = boxed_result.err().unwrap();
        println!("{} {}", error_message, app_id);

        let is_not_steam_unsuccessful_response = error_message != "steampowered api returned failed response";
        let is_not_invalid_utf8_sequence = error_message != "invalid utf-8 sequence";
        let no_response_from_api = error_message == "no response from API";

        if (is_not_steam_unsuccessful_response && is_not_invalid_utf8_sequence) || no_response_from_api {
            println!("result is not ok for app id {}, retry in 1 min ", app_id);

            let one_minute = time::Duration::from_secs(60);
            thread::sleep(one_minute);

            get_app_details(app_id);
        }



    }
}
