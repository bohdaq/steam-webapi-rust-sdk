use crate::store_steampowered_com::appdetails;

#[test]
fn test_get() {
    let app_id = 570;
    let app = appdetails::get(app_id).unwrap();

    assert_eq!(app.name, "Dota 2".to_string());
}

#[test]
fn test_get_cached() {
    let app_id = 730;
    let app = appdetails::get_cached(app_id).unwrap();

    assert_eq!(app.name, "Counter-Strike: Global Offensive".to_string());
}