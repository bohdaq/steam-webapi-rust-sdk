use crate::store_steampowered_com::appdetails;

#[test]
fn test_make_api_call() {
    let response = appdetails::get(570);
}