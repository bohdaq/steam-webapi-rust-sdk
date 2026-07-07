//! # Steam Web API Rust SDK
//!
//! `steam-webapi-rust-sdk` is a set of utility functions to access Steam Web API.
//!
//! In order to use this library make sure to set STEAM_WEB_API_KEY system environment variable.
//!
//! The library itself tries to minimize number of networks calls through the caching relevant
//! responses to the 'steam-webapi-cache' folder.
//!
//! There is already prebuilt cache for all steam apps, in order to use it,
//! simply clone [steam-webapi-cache](https://github.com/bohdaq/steam-webapi-cache)
//! into the root folder of your project.

use crate::idota2match_570::get_heroes::Hero;
use crate::idota2match_570::get_league_listing::League;
use crate::idota2match_570::get_live_league_games::LiveLeagueGame;
use crate::idota2match_570::get_match_details::MatchResult;
use crate::idota2match_570::get_match_history::ResponseMatchHistory;
use crate::idota2match_570::get_team_info_by_team_id::TeamInfo;
use crate::isteam_apps::get_app_list::SteamApp;
use crate::isteam_news::get_news_for_app::NewsItem;
use crate::isteam_user::get_friend_list::Friend;
use crate::isteam_user::get_player_bans::PlayerBans;
use crate::isteam_user::get_player_summaries::PlayerSummary;
use crate::isteam_user::resolve_vanity_url::VanityUrlResolution;
use crate::isteam_user_stats::get_global_achievement_percentages_for_app::AchievementPercentage;
use crate::isteam_user_stats::get_player_achievements::PlayerAchievements;
use crate::isteam_user_stats::get_schema_for_game::GameSchema;
use crate::isteam_user_stats::get_user_stats_for_game::UserStatsForGame;
use crate::iplayer_service::get_badges::Badges;
use crate::iplayer_service::get_owned_games::OwnedGames;
use crate::iplayer_service::get_recently_played_games::RecentlyPlayedGames;
use crate::store_steampowered_com::appdetails::SteamAppDetails;

pub mod util;
pub mod isteam_apps;
pub mod isteam_user;
pub mod isteam_user_stats;
pub mod isteam_news;
pub mod iplayer_service;
pub mod store_steampowered_com;
pub mod idota2match_570;

#[cfg(test)]
mod tests;

/// Retrieves details for the given app id from the local cache. It may return an error
/// if requested resource is absent, malformed or not readable from local cache.
///
/// # Examples
///
/// ```
/// let app_id = 570;
/// let boxed_result = steam_webapi_rust_sdk::get_cached_app_details(app_id);
/// if boxed_result.is_ok() {
///     let app_details = boxed_result.unwrap();
///     println!("result is ok for {} app id {}", app_details.name, app_details.app_id);
///
/// } else {
///     let error_message = boxed_result.err().unwrap();
///     println!("{} {}", error_message, app_id);
///
/// };
/// ```
pub fn get_cached_app_details(app_id: i64) -> Result<SteamAppDetails, String> {
    let boxed_result = store_steampowered_com::appdetails::get_cached(app_id);
    boxed_result
}

/// Retrieves details for the given app id. It will make an API call to Steam and cache response.
/// It may return an error if API responded with error response. As an example it may be exceeding
/// the limit of calls from one IP address or if the response contains not valid UTF-8 characters.
/// Usually Steam API allows 200 requests from single IP address within 5 minutes range.
///
/// # Examples
///
/// ```
/// let app_id = 570;
/// let boxed_result = steam_webapi_rust_sdk::get_app_details(app_id);
/// if boxed_result.is_ok() {
///     let app_details = boxed_result.unwrap();
///     println!("result is ok for {} app id {}", app_details.name, app_details.app_id);
///
/// } else {
///     let error_message = boxed_result.err().unwrap();
///     println!("{} {}", error_message, app_id);
///
///     let is_steam_unsuccessful_response = error_message == "steampowered api returned failed response";
///     let is_invalid_utf8_sequence = error_message == "invalid utf-8 sequence";
///     let no_response_from_api = error_message == "no response from API";
///     let exceeded_api_calls_limit = (!is_steam_unsuccessful_response && !is_invalid_utf8_sequence) || no_response_from_api;
///
///     // you can do a retry or continue execution...
/// };
/// ```
pub fn get_app_details(app_id: i64) -> Result<SteamAppDetails, String> {
    let boxed_result = store_steampowered_com::appdetails::get(app_id);
    boxed_result
}

/// Retrieves list of apps available on Steam. Each item consists of 2 fields: appid and name
///
/// # Examples
///
/// ```
/// let steam_app_list = steam_webapi_rust_sdk::get_app_list().unwrap();
///
/// assert!(steam_app_list.len()>0);
/// let steam_app = steam_app_list.get(0).unwrap();
/// assert!(steam_app.appid > 0);
///
/// assert!(steam_app.name.len() > 0);
/// ```
pub fn get_app_list() -> Result<Vec<SteamApp>, String> {
    let boxed_result = isteam_apps::get_app_list::get();
    boxed_result
}


/// Retrieves list of apps available on Steam. First tries to get it from local cache.
/// Each item consists of 2 fields: appid and name
///
/// # Examples
///
/// ```
/// let steam_app_list = steam_webapi_rust_sdk::get_cached_app_list().unwrap();
///
/// assert!(steam_app_list.len()>0);
/// let steam_app = steam_app_list.get(0).unwrap();
/// assert!(steam_app.appid > 0);
///
/// assert!(steam_app.name.len() > 0);
/// ```
pub fn get_cached_app_list() -> Result<Vec<SteamApp>, String> {
    let boxed_result = isteam_apps::get_app_list::get_cached();
    boxed_result
}

/// Retrieves list of matches from Dota2
///
/// # Examples
///
/// ```
/// let boxed_dota2_match_list = steam_webapi_rust_sdk::get_dota2_match_history(
///     Some(76561197960361544),
///     None,
///     None,
///     None,
///     None,
///     None,
///     None
/// );
///
/// assert!(boxed_dota2_match_list.is_ok());
/// if boxed_dota2_match_list.is_ok() {
///     let dota2_match_list = boxed_dota2_match_list.unwrap();
///     assert!(dota2_match_list.matches.len()>0);
/// }
///
/// ```
pub fn get_dota2_match_history(account_id: Option<i64>,
                               game_mode: Option<u8>,
                               skill: Option<u8>,
                               min_players: Option<u32>,
                               start_at_match_id: Option<i64>,
                               matches_requested: Option<u32>,
                               tournament_games_only: Option<bool>)
    -> Result<ResponseMatchHistory, String> {
    idota2match_570::get_dota2_match_history(
        account_id,
        game_mode,
        skill,
        min_players,
        start_at_match_id,
        matches_requested,
        tournament_games_only
    )

}

/// Retrieves match details for the given Dota2 match id. It will make an API call to Steam and
/// cache the response.
///
/// # Examples
///
/// ```
/// let match_id = 1461414523;
/// let boxed_match_result = steam_webapi_rust_sdk::get_dota2_match_details(match_id);
///
/// assert!(boxed_match_result.is_ok());
/// if boxed_match_result.is_ok() {
///     let match_result = boxed_match_result.unwrap();
///     assert_eq!(10, match_result.players.len());
/// }
/// ```
pub fn get_dota2_match_details(match_id: u64) -> Result<MatchResult, String> {
    idota2match_570::get_dota2_match_details(match_id)
}

/// Retrieves match details for the given Dota2 match id from the local cache. It may return an
/// error if requested resource is absent, malformed or not readable from local cache.
///
/// # Examples
///
/// ```
/// let match_id = 1461414523;
/// let boxed_match_result = steam_webapi_rust_sdk::get_cached_dota2_match_details(match_id);
/// if boxed_match_result.is_ok() {
///     let match_result = boxed_match_result.unwrap();
///     println!("result is ok for match id {}", match_result.match_id);
///
/// } else {
///     let error_message = boxed_match_result.err().unwrap();
///     println!("{} {}", error_message, match_id);
///
/// };
/// ```
pub fn get_cached_dota2_match_details(match_id: u64) -> Result<MatchResult, String> {
    idota2match_570::get_cached_dota2_match_details(match_id)
}

/// Retrieves the list of all Dota2 heroes.
///
/// # Examples
///
/// ```no_run
/// let boxed_heroes = steam_webapi_rust_sdk::get_dota2_heroes(Some("en".to_string()));
/// assert!(boxed_heroes.is_ok());
/// ```
pub fn get_dota2_heroes(language: Option<String>) -> Result<Vec<Hero>, String> {
    idota2match_570::get_dota2_heroes(language)
}

/// Retrieves the list of Dota2 leagues.
///
/// # Examples
///
/// ```no_run
/// let boxed_leagues = steam_webapi_rust_sdk::get_dota2_league_listing(None);
/// assert!(boxed_leagues.is_ok());
/// ```
pub fn get_dota2_league_listing(language: Option<String>) -> Result<Vec<League>, String> {
    idota2match_570::get_dota2_league_listing(language)
}

/// Retrieves currently live Dota2 league games.
///
/// # Examples
///
/// ```no_run
/// let boxed_games = steam_webapi_rust_sdk::get_dota2_live_league_games();
/// assert!(boxed_games.is_ok());
/// ```
pub fn get_dota2_live_league_games() -> Result<Vec<LiveLeagueGame>, String> {
    idota2match_570::get_dota2_live_league_games()
}

/// Retrieves Dota2 team info, paginated by team id.
///
/// # Examples
///
/// ```no_run
/// let boxed_teams = steam_webapi_rust_sdk::get_dota2_team_info_by_team_id(Some(1), Some(100));
/// assert!(boxed_teams.is_ok());
/// ```
pub fn get_dota2_team_info_by_team_id(start_at_team_id: Option<u64>, teams_requested: Option<u32>) -> Result<Vec<TeamInfo>, String> {
    idota2match_570::get_dota2_team_info_by_team_id(start_at_team_id, teams_requested)
}

/// Retrieves public profile summaries (up to 100 SteamIDs per call) via `ISteamUser/GetPlayerSummaries`.
///
/// # Examples
///
/// ```no_run
/// let boxed_summaries = steam_webapi_rust_sdk::get_player_summaries(vec![76561197960361544]);
/// assert!(boxed_summaries.is_ok());
/// ```
pub fn get_player_summaries(steamids: Vec<u64>) -> Result<Vec<PlayerSummary>, String> {
    isteam_user::get_player_summaries::get(steamids)
}

/// Retrieves a Steam account's friend list. Returns an error if the profile's friends list is
/// not public.
///
/// # Examples
///
/// ```no_run
/// let boxed_friends = steam_webapi_rust_sdk::get_friend_list(76561197960361544, None);
/// assert!(boxed_friends.is_ok());
/// ```
pub fn get_friend_list(steamid: u64, relationship: Option<String>) -> Result<Vec<Friend>, String> {
    isteam_user::get_friend_list::get(steamid, relationship)
}

/// Retrieves VAC/game/community ban status for up to 100 SteamIDs.
///
/// # Examples
///
/// ```no_run
/// let boxed_bans = steam_webapi_rust_sdk::get_player_bans(vec![76561197960361544]);
/// assert!(boxed_bans.is_ok());
/// ```
pub fn get_player_bans(steamids: Vec<u64>) -> Result<Vec<PlayerBans>, String> {
    isteam_user::get_player_bans::get(steamids)
}

/// Resolves a Steam Community vanity URL (e.g. `steamcommunity.com/id/<vanity>`) to a 64-bit SteamID.
///
/// # Examples
///
/// ```no_run
/// let boxed_resolution = steam_webapi_rust_sdk::resolve_vanity_url("gabelogannewell".to_string(), None);
/// assert!(boxed_resolution.is_ok());
/// ```
pub fn resolve_vanity_url(vanity_url: String, url_type: Option<u8>) -> Result<VanityUrlResolution, String> {
    isteam_user::resolve_vanity_url::get(vanity_url, url_type)
}

/// Retrieves the games a Steam account owns, and optionally their playtime.
///
/// # Examples
///
/// ```no_run
/// let boxed_games = steam_webapi_rust_sdk::get_owned_games(76561197960361544, Some(true), Some(true));
/// assert!(boxed_games.is_ok());
/// ```
pub fn get_owned_games(steamid: u64, include_appinfo: Option<bool>, include_played_free_games: Option<bool>) -> Result<OwnedGames, String> {
    iplayer_service::get_owned_games::get(steamid, include_appinfo, include_played_free_games)
}

/// Retrieves games played by a Steam account in the last two weeks.
///
/// # Examples
///
/// ```no_run
/// let boxed_games = steam_webapi_rust_sdk::get_recently_played_games(76561197960361544, None);
/// assert!(boxed_games.is_ok());
/// ```
pub fn get_recently_played_games(steamid: u64, count: Option<u32>) -> Result<RecentlyPlayedGames, String> {
    iplayer_service::get_recently_played_games::get(steamid, count)
}

/// Retrieves a Steam account's Steam level.
///
/// # Examples
///
/// ```no_run
/// let boxed_level = steam_webapi_rust_sdk::get_steam_level(76561197960361544);
/// assert!(boxed_level.is_ok());
/// ```
pub fn get_steam_level(steamid: u64) -> Result<u64, String> {
    iplayer_service::get_steam_level::get(steamid)
}

/// Retrieves a Steam account's badges and badge XP progress.
///
/// # Examples
///
/// ```no_run
/// let boxed_badges = steam_webapi_rust_sdk::get_badges(76561197960361544);
/// assert!(boxed_badges.is_ok());
/// ```
pub fn get_badges(steamid: u64) -> Result<Badges, String> {
    iplayer_service::get_badges::get(steamid)
}

/// Retrieves a Steam account's achievement status for a given app. Returns an error if the
/// account's stats for the game aren't public.
///
/// # Examples
///
/// ```no_run
/// let boxed_achievements = steam_webapi_rust_sdk::get_player_achievements(76561197960361544, 440, Some("english".to_string()));
/// assert!(boxed_achievements.is_ok());
/// ```
pub fn get_player_achievements(steamid: u64, appid: i64, language: Option<String>) -> Result<PlayerAchievements, String> {
    isteam_user_stats::get_player_achievements::get(steamid, appid, language)
}

/// Retrieves a Steam account's stats and achievements for a given app.
///
/// # Examples
///
/// ```no_run
/// let boxed_stats = steam_webapi_rust_sdk::get_user_stats_for_game(76561197960361544, 440);
/// assert!(boxed_stats.is_ok());
/// ```
pub fn get_user_stats_for_game(steamid: u64, appid: i64) -> Result<UserStatsForGame, String> {
    isteam_user_stats::get_user_stats_for_game::get(steamid, appid)
}

/// Retrieves the stat and achievement schema (names, display names, descriptions) for a given app.
///
/// # Examples
///
/// ```no_run
/// let boxed_schema = steam_webapi_rust_sdk::get_schema_for_game(440);
/// assert!(boxed_schema.is_ok());
/// ```
pub fn get_schema_for_game(appid: i64) -> Result<GameSchema, String> {
    isteam_user_stats::get_schema_for_game::get(appid)
}

/// Retrieves the global completion percentage for each achievement of a given app. Does not
/// require a Steam Web API key.
///
/// # Examples
///
/// ```no_run
/// let boxed_percentages = steam_webapi_rust_sdk::get_global_achievement_percentages_for_app(440);
/// assert!(boxed_percentages.is_ok());
/// ```
pub fn get_global_achievement_percentages_for_app(appid: i64) -> Result<Vec<AchievementPercentage>, String> {
    isteam_user_stats::get_global_achievement_percentages_for_app::get(appid)
}

/// Retrieves the current number of players in-game for a given app. Does not require a Steam
/// Web API key.
///
/// # Examples
///
/// ```no_run
/// let boxed_count = steam_webapi_rust_sdk::get_number_of_current_players(570);
/// assert!(boxed_count.is_ok());
/// ```
pub fn get_number_of_current_players(appid: i64) -> Result<i64, String> {
    isteam_user_stats::get_number_of_current_players::get(appid)
}

/// Retrieves news items for a given app. Does not require a Steam Web API key.
///
/// # Examples
///
/// ```no_run
/// let boxed_news = steam_webapi_rust_sdk::get_news_for_app(440, Some(3), Some(300));
/// assert!(boxed_news.is_ok());
/// ```
pub fn get_news_for_app(appid: i64, count: Option<u32>, maxlength: Option<u32>) -> Result<Vec<NewsItem>, String> {
    isteam_news::get_news_for_app::get(appid, count, maxlength)
}

/// Converts given 32 bit Steam account id to 64 bit
///
/// # Examples
///
/// ```
/// use steam_webapi_rust_sdk::convert_32bit_account_id_to_64bit;
///
/// let _32bit_id = 95816;
/// let converted = convert_32bit_account_id_to_64bit(_32bit_id);
///
/// let expected_id = 76561197960361544;
/// assert_eq!(expected_id, converted);
///
/// ```
pub fn convert_32bit_account_id_to_64bit(account_id_32bit: i64) -> i64 {
    let valves_magic_constant = 76561197960265728;
    let mut converted_to_64_bit = account_id_32bit;
    converted_to_64_bit += valves_magic_constant;
    converted_to_64_bit
}


/// Converts given 64 bit Steam account id to 32 bit
///
/// # Examples
///
/// ```
/// use steam_webapi_rust_sdk::convert_64bit_account_id_to_32bit;
///
/// let _64bit_id = 76561197960361544;
/// let converted = convert_64bit_account_id_to_32bit(_64bit_id);
///
/// let expected_id = 95816;
/// assert_eq!(expected_id, converted);
///
/// ```
pub fn convert_64bit_account_id_to_32bit(account_id_32bit: i64) -> i64 {
    let valves_magic_constant = 76561197960265728;
    let mut converted_to_32_bit = account_id_32bit;
    converted_to_32_bit -= valves_magic_constant;
    converted_to_32_bit
}


pub(crate) fn get_scheme() -> String {
    "https".to_string()
}

pub(crate) fn get_host() -> String {
    let host = "api.steampowered.com".to_string();
    host
}

pub(crate) fn make_api_call(url: String) -> Result<String, String> {

    let boxed_response = minreq::get(url).send();
    if boxed_response.is_err() {
        return Err("Operation timed out (API call)".to_string());
    }

    let raw_response : Vec<u8> = boxed_response.unwrap().into_bytes();

    let response_string_boxed = String::from_utf8(raw_response);
    if response_string_boxed.is_err() {
        let error_message = response_string_boxed.err().unwrap().to_string();
        if error_message == "invalid utf-8 sequence of 1 bytes from index 1" {
            return Err("no response from API".to_string());
        }
        return Err("invalid utf-8 sequence".to_string());
    }
    let response_string: String = response_string_boxed.unwrap();

    Ok(response_string)
}