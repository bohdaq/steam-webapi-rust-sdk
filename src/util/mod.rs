use std::env;

pub fn getSteamWebAPIKey() -> String {
    let boxed_steam_web_api_key = env::var("STEAM_WEBAPI_KEY");
    if boxed_steam_web_api_key.is_err() {
        println!("To use this SDK please specify STEAM_WEBAPI_KEY environment variable");
        return "".to_string();
    }
    let steam_web_api_key = boxed_steam_web_api_key.unwrap();
    let _key = ["STEAM_WEB_API_KEY is ", &steam_web_api_key].join("");

    println!("{}", _key);

    return steam_web_api_key;
}