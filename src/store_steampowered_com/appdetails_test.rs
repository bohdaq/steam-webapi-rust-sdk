use crate::store_steampowered_com::appdetails;

#[test]
fn test_make_api_call_details_dota() {
    let response = appdetails::get(570);
}

#[test]
fn test_make_api_call_details_csgo() {
    let response = appdetails::get(730);
}