use crate::idota2match_570::get_heroes::Hero;
use crate::idota2match_570::get_league_listing::League;
use crate::idota2match_570::get_live_league_games::LiveLeagueGame;
use crate::idota2match_570::get_match_details::MatchResult;
use crate::idota2match_570::get_match_history::ResponseMatchHistory;
use crate::idota2match_570::get_team_info_by_team_id::TeamInfo;

pub mod get_match_history;
pub mod get_match_details;
pub mod get_heroes;
pub mod get_league_listing;
pub mod get_live_league_games;
pub mod get_team_info_by_team_id;

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

pub fn get_dota2_match_details(match_id: u64) -> Result<MatchResult, String> {
    get_match_details::get(match_id)
}

pub fn get_cached_dota2_match_details(match_id: u64) -> Result<MatchResult, String> {
    get_match_details::get_cached(match_id)
}

pub fn get_dota2_heroes(language: Option<String>) -> Result<Vec<Hero>, String> {
    get_heroes::get(language)
}

pub fn get_dota2_league_listing(language: Option<String>) -> Result<Vec<League>, String> {
    get_league_listing::get(language)
}

pub fn get_dota2_live_league_games() -> Result<Vec<LiveLeagueGame>, String> {
    get_live_league_games::get()
}

pub fn get_dota2_team_info_by_team_id(start_at_team_id: Option<u64>, teams_requested: Option<u32>) -> Result<Vec<TeamInfo>, String> {
    get_team_info_by_team_id::get(start_at_team_id, teams_requested)
}