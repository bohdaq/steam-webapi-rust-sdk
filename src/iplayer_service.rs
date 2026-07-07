pub mod get_owned_games;
pub mod get_recently_played_games;
pub mod get_steam_level;
pub mod get_badges;

pub fn get_interface() -> String {
    "IPlayerService".to_string()
}
