use crate::{get_app_details, get_cached_app_details};

#[test]
fn test_get_app_details() {
    let app_id = 570;
    let app = get_app_details(app_id).unwrap();

    assert_eq!(app.name, "Dota 2".to_string());
}

#[test]
fn test_get_cached_app_details() {
    let app_id = 730;
    let app = get_cached_app_details(app_id).unwrap();

    assert_eq!(app.name, "Counter-Strike: Global Offensive".to_string());
}