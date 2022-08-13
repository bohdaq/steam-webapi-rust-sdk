pub mod get_app_list;

#[cfg(test)]
mod tests;

pub fn get_interface() -> String {
    "ISteamApps".to_string()
}