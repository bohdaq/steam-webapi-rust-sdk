pub mod get_app_list;

#[cfg(test)]
mod get_app_list_test;

pub fn get_interface() -> String {
    "ISteamApps".to_string()
}