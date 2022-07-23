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


pub mod util;
pub mod isteam_apps;
pub mod store_steampowered_com;

fn main() {
    println!("Steam Web API Rust SDK");

    let app_list = isteam_apps::get_app_list::get();
    for app in app_list {
        let boxed_result = store_steampowered_com::appdetails::get(app.appid);
        if boxed_result.is_ok() {
            println!("result is ok")
        } else {
            println!("result is not ok")
        }
    }


}
