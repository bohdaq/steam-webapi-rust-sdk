pub mod get_player_summaries;
pub mod get_friend_list;
pub mod get_player_bans;
pub mod resolve_vanity_url;

pub fn get_interface() -> String {
    "ISteamUser".to_string()
}
