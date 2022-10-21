use std::fs::read_to_string;
use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::idota2match_570::get_match_history::{GAME_MODE, get_api_url, parse_response, PLAYER_SKILL};
use crate::store_steampowered_com::appdetails::parse_api_call_result;
use crate::util::get_steam_web_api_key;

#[test]
fn modes() {
    assert_eq!(0, GAME_MODE.none);
    assert_eq!(1, GAME_MODE.all_pick);
    assert_eq!(2, GAME_MODE.captains_mode);
    assert_eq!(3, GAME_MODE.random_draft);
    assert_eq!(4, GAME_MODE.single_draft);
    assert_eq!(5, GAME_MODE.all_random);
    assert_eq!(6, GAME_MODE.intro);
    assert_eq!(7, GAME_MODE.diretide);
    assert_eq!(8, GAME_MODE.reverse_captains_mode);
    assert_eq!(9, GAME_MODE.the_greeviling);
    assert_eq!(10, GAME_MODE.tutorial);
    assert_eq!(11, GAME_MODE.mid_only);
    assert_eq!(12, GAME_MODE.least_played);
    assert_eq!(13, GAME_MODE.new_player_pool);
    assert_eq!(14, GAME_MODE.compendium_matchmaking);
    assert_eq!(16, GAME_MODE.captains_draft);
}

#[test]
fn skill() {
    assert_eq!(0, PLAYER_SKILL.any);
    assert_eq!(1, PLAYER_SKILL.normal);
    assert_eq!(2, PLAYER_SKILL.high);
    assert_eq!(3, PLAYER_SKILL.very_high);
}

#[test]
fn api_url_no_options() {
    let api_url = get_api_url(
        None,
        None,
        None,
        None,
        None,
        None,
        None
    );

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/IDOTA2Match_570/GetMatchHistory/v1", components.path);

    let params = components.query.unwrap();
    let boxed_account_id = params.get("account_id");
    assert_eq!(None, boxed_account_id);

    let boxed_game_mode = params.get("game_mode");
    assert_eq!(None, boxed_game_mode);

    let boxed_skill = params.get("skill");
    assert_eq!(None, boxed_skill);

    let boxed_min_players = params.get("min_players");
    assert_eq!(None, boxed_min_players);

    let boxed_start_at_match_id = params.get("start_at_match_id");
    assert_eq!(None, boxed_start_at_match_id);

    let boxed_matches_requested = params.get("matches_requested");
    assert_eq!(None, boxed_matches_requested);

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());


}


#[test]
fn api_url_options() {
    let api_url = get_api_url(
        Some(76561197960361544),
        Some(GAME_MODE.all_pick),
        None,
        None,
        None,
        None,
        None
    );

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/IDOTA2Match_570/GetMatchHistory/v1", components.path);

    let params = components.query.unwrap();
    let boxed_account_id = params.get("account_id");
    assert_eq!(76561197960361544, boxed_account_id.unwrap().parse::<u64>().unwrap());

    let boxed_game_mode = params.get("game_mode");
    assert_eq!(GAME_MODE.all_pick, boxed_game_mode.unwrap().parse::<u8>().unwrap());

    let boxed_skill = params.get("skill");
    assert_eq!(None, boxed_skill);

    let boxed_min_players = params.get("min_players");
    assert_eq!(None, boxed_min_players);

    let boxed_start_at_match_id = params.get("start_at_match_id");
    assert_eq!(None, boxed_start_at_match_id);

    let boxed_matches_requested = params.get("matches_requested");
    assert_eq!(None, boxed_matches_requested);

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());


}

#[test]
fn parse(){
    let filepath = "test/idota2match_570/get_match_history/76561197960361544.json";
    let boxed_read = read_to_string(filepath);

    let response = boxed_read.unwrap();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let response_match_history = boxed_parse.unwrap();
    assert_eq!(1, response_match_history.status);
    assert_eq!(1, response_match_history.num_results);
    assert_eq!(500, response_match_history.total_results);
    assert_eq!(499, response_match_history.results_remaining);
    assert_eq!(1, response_match_history.matches.len());

    let matches = response_match_history.matches;
    let match_1 = matches.get(0).unwrap();

    assert_eq!(5066503471, match_1.match_id);
    assert_eq!(4250346899, match_1.match_seq_num);
    assert_eq!(1570905295, match_1.start_time);
    assert_eq!(0, match_1.lobby_type);
    assert_eq!(0, match_1.radiant_team_id);
    assert_eq!(0, match_1.dire_team_id);

    assert_eq!(10, match_1.players.len());

    let player_1 = match_1.players.get(0).unwrap();
    assert_eq!(178918873, player_1.account_id);
    assert_eq!(0, player_1.player_slot);
    assert_eq!(0, player_1.team_number);
    assert_eq!(0, player_1.team_slot);
    assert_eq!(97, player_1.hero_id);

    let player_2 = match_1.players.get(1).unwrap();
    assert_eq!(4294967295, player_2.account_id);
    assert_eq!(1, player_2.player_slot);
    assert_eq!(0, player_2.team_number);
    assert_eq!(1, player_2.team_slot);
    assert_eq!(106, player_2.hero_id);

    let player_3 = match_1.players.get(2).unwrap();
    assert_eq!(184106424, player_3.account_id);
    assert_eq!(2, player_3.player_slot);
    assert_eq!(0, player_3.team_number);
    assert_eq!(2, player_3.team_slot);
    assert_eq!(27, player_3.hero_id);

    let player_4 = match_1.players.get(3).unwrap();
    assert_eq!(4294967295, player_4.account_id);
    assert_eq!(3, player_4.player_slot);
    assert_eq!(0, player_4.team_number);
    assert_eq!(3, player_4.team_slot);
    assert_eq!(74, player_4.hero_id);

    let player_5 = match_1.players.get(4).unwrap();
    assert_eq!(93497086, player_5.account_id);
    assert_eq!(4, player_5.player_slot);
    assert_eq!(0, player_5.team_number);
    assert_eq!(4, player_5.team_slot);
    assert_eq!(19, player_5.hero_id);

    let player_6 = match_1.players.get(5).unwrap();
    assert_eq!(75611603, player_6.account_id);
    assert_eq!(128, player_6.player_slot);
    assert_eq!(1, player_6.team_number);
    assert_eq!(0, player_6.team_slot);
    assert_eq!(14, player_6.hero_id);

    let player_7 = match_1.players.get(6).unwrap();
    assert_eq!(95816, player_7.account_id);
    assert_eq!(129, player_7.player_slot);
    assert_eq!(1, player_7.team_number);
    assert_eq!(1, player_7.team_slot);
    assert_eq!(8, player_7.hero_id);

    let player_8 = match_1.players.get(7).unwrap();
    assert_eq!(4294967295, player_8.account_id);
    assert_eq!(130, player_8.player_slot);
    assert_eq!(1, player_8.team_number);
    assert_eq!(2, player_8.team_slot);
    assert_eq!(101, player_8.hero_id);

    let player_9 = match_1.players.get(8).unwrap();
    assert_eq!(32748914, player_9.account_id);
    assert_eq!(131, player_9.player_slot);
    assert_eq!(1, player_9.team_number);
    assert_eq!(3, player_9.team_slot);
    assert_eq!(119, player_9.hero_id);

    let player_10 = match_1.players.get(9).unwrap();
    assert_eq!(4294967295, player_10.account_id);
    assert_eq!(132, player_10.player_slot);
    assert_eq!(1, player_10.team_number);
    assert_eq!(4, player_10.team_slot);
    assert_eq!(59, player_10.hero_id);
}
