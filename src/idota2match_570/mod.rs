use crate::idota2match_570::get_match_history::ResponseMatchHistory;

pub mod get_match_history;
pub mod get_match_details;

pub fn get_interface() -> String {
    "IDOTA2Match_570".to_string()
}

pub fn get_dota2_match_history(account_id: Option<i64>,
                               game_mode: Option<u8>,
                               skill: Option<u8>,
                               min_players: Option<u32>,
                               start_at_match_id: Option<i64>,
                               matches_requested: Option<u32>,
                               tournament_games_only: Option<bool>)
-> Result<ResponseMatchHistory, String>{
    get_match_history::get(
        account_id,
        game_mode,
        skill,
        min_players,
        start_at_match_id,
        matches_requested,
        tournament_games_only
    )
}