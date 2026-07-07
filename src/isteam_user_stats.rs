pub mod get_player_achievements;
pub mod get_user_stats_for_game;
pub mod get_schema_for_game;
pub mod get_global_achievement_percentages_for_app;
pub mod get_number_of_current_players;

pub fn get_interface() -> String {
    "ISteamUserStats".to_string()
}
