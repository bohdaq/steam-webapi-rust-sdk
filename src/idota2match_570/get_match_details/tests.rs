use std::fs::read_to_string;
use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::idota2match_570::get_match_details::{get_api_url, get_resource_filepath, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url() {
    let api_url = get_api_url(1461414523);

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/IDOTA2Match_570/GetMatchDetails/v1", components.path);

    let params = components.query.unwrap();
    let boxed_match_id = params.get("match_id");
    assert_eq!(1461414523, boxed_match_id.unwrap().parse::<u64>().unwrap());

    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn resource_filepath() {
    let filepath = get_resource_filepath(1461414523);

    assert_eq!("steam-webapi-cache/IDOTA2Match_570-GetMatchDetails-v1-1461414523.json", filepath);
}

#[test]
fn parse() {
    let filepath = "test/idota2match_570/get_match_details/1461414523.json";
    let boxed_read = read_to_string(filepath);

    let response = boxed_read.unwrap();

    let boxed_parse = parse_response(response, 1461414523);
    assert!(boxed_parse.is_ok());

    let match_result = boxed_parse.unwrap();

    assert_eq!(false, match_result.radiant_win);
    assert_eq!(2675, match_result.duration);
    assert_eq!(0, match_result.pre_game_duration);
    assert_eq!(1431220172, match_result.start_time);
    assert_eq!(1461414523, match_result.match_id);
    assert_eq!(1310985420, match_result.match_seq_num);
    assert_eq!(32, match_result.tower_status_radiant);
    assert_eq!(1958, match_result.tower_status_dire);
    assert_eq!(12, match_result.barracks_status_radiant);
    assert_eq!(63, match_result.barracks_status_dire);
    assert_eq!(123, match_result.cluster);
    assert_eq!(109, match_result.first_blood_time);
    assert_eq!(7, match_result.lobby_type);
    assert_eq!(10, match_result.human_players);
    assert_eq!(0, match_result.leagueid);
    assert_eq!(0, match_result.positive_votes);
    assert_eq!(0, match_result.negative_votes);
    assert_eq!(22, match_result.game_mode);
    assert_eq!(0, match_result.flags);
    assert_eq!(0, match_result.engine);
    assert_eq!(0, match_result.radiant_score);
    assert_eq!(0, match_result.dire_score);

    assert_eq!(10, match_result.players.len());

    let player_1 = match_result.players.get(0).unwrap();
    assert_eq!(172120682, player_1.account_id);
    assert_eq!(0, player_1.player_slot);
    assert_eq!(0, player_1.team_number);
    assert_eq!(0, player_1.team_slot);
    assert_eq!(12, player_1.hero_id);
    assert_eq!(61, player_1.item_0);
    assert_eq!(185, player_1.item_1);
    assert_eq!(113, player_1.item_2);
    assert_eq!(174, player_1.item_3);
    assert_eq!(212, player_1.item_4);
    assert_eq!(63, player_1.item_5);
    assert_eq!(0, player_1.backpack_0);
    assert_eq!(0, player_1.backpack_1);
    assert_eq!(0, player_1.backpack_2);
    assert_eq!(0, player_1.item_neutral);
    assert_eq!(7, player_1.kills);
    assert_eq!(10, player_1.deaths);
    assert_eq!(3, player_1.assists);
    assert_eq!(0, player_1.leaver_status);
    assert_eq!(186, player_1.last_hits);
    assert_eq!(4, player_1.denies);
    assert_eq!(371, player_1.gold_per_min);
    assert_eq!(457, player_1.xp_per_min);
    assert_eq!(19, player_1.level);
    assert_eq!(10772, player_1.net_worth);
    assert_eq!(0, player_1.aghanims_scepter);
    assert_eq!(0, player_1.aghanims_shard);
    assert_eq!(0, player_1.moonshard);

    let player_6 = match_result.players.get(5).unwrap();
    assert_eq!(95816, player_6.account_id);
    assert_eq!(128, player_6.player_slot);
    assert_eq!(1, player_6.team_number);
    assert_eq!(0, player_6.team_slot);
    assert_eq!(112, player_6.hero_id);
    assert_eq!(4, player_6.kills);
    assert_eq!(4, player_6.deaths);
    assert_eq!(19, player_6.assists);

    let player_10 = match_result.players.get(9).unwrap();
    assert_eq!(115823162, player_10.account_id);
    assert_eq!(132, player_10.player_slot);
    assert_eq!(1, player_10.team_number);
    assert_eq!(4, player_10.team_slot);
    assert_eq!(17, player_10.hero_id);
    assert_eq!(18, player_10.kills);
    assert_eq!(8, player_10.deaths);
    assert_eq!(14, player_10.assists);
}

#[test]
fn parse_missing_result() {
    let boxed_parse = parse_response("{}".to_string(), 1461414523);
    assert!(boxed_parse.is_err());
    assert_eq!("response does not contain a result", boxed_parse.err().unwrap());
}

#[test]
fn parse_error_response() {
    let response = "{\"result\":{\"error\":\"Match ID not found\"}}".to_string();
    let boxed_parse = parse_response(response, 1461414523);
    assert!(boxed_parse.is_err());
    assert_eq!("Match ID not found", boxed_parse.err().unwrap());
}
