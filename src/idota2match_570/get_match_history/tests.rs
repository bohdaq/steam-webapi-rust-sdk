use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::idota2match_570::get_match_history::{GAME_MODE, get_api_url, PLAYER_SKILL};
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