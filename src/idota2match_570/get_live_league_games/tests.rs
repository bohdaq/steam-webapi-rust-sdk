use url_build_parse::parse_url;
use crate::{get_host, get_scheme};
use crate::idota2match_570::get_live_league_games::{get_api_url, parse_response};
use crate::util::get_steam_web_api_key;

#[test]
fn api_url() {
    let api_url = get_api_url();

    let components = parse_url(api_url.as_str()).unwrap();

    assert_eq!(get_scheme(), components.scheme);
    assert_eq!(get_host(), components.authority.unwrap().host);
    assert_eq!("/IDOTA2Match_570/GetLiveLeagueGames/v1", components.path);

    let params = components.query.unwrap();
    let boxed_key = params.get("key");
    assert_eq!(get_steam_web_api_key(), boxed_key.unwrap().to_string());
}

#[test]
fn parse() {
    let response = r#"{
        "result": {
            "games": [
                {
                    "players": [
                        {"account_id": 12345, "hero_id": 1, "team": 0}
                    ],
                    "radiant_team": {"team_name": "Radiant Squad", "team_id": 1, "team_logo": 999, "complete": 1},
                    "dire_team": {"team_name": "Dire Squad", "team_id": 2, "team_logo": 888, "complete": 1},
                    "lobby_id": 111,
                    "match_id": 222,
                    "spectators": 100,
                    "league_id": 14268,
                    "series_type": 1,
                    "radiant_series_wins": 0,
                    "dire_series_wins": 1
                }
            ]
        }
    }"#.to_string();

    let boxed_parse = parse_response(response);
    assert!(boxed_parse.is_ok());

    let games = boxed_parse.unwrap();
    assert_eq!(1, games.len());

    let game = games.get(0).unwrap();
    assert_eq!(222, game.match_id);
    assert_eq!(1, game.players.len());
    assert_eq!(12345, game.players.get(0).unwrap().account_id);
    assert_eq!("Radiant Squad", game.radiant_team.team_name);
    assert_eq!(true, game.radiant_team.complete);
    assert_eq!("Dire Squad", game.dire_team.team_name);
    assert_eq!(1, game.dire_series_wins);
}

#[test]
fn parse_missing_games() {
    let boxed_parse = parse_response("{}".to_string());
    assert!(boxed_parse.is_err());
}
