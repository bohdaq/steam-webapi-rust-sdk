use crate::idota2match_570::get_match_history::GAME_MODE;

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