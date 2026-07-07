# steam-webapi-rust-sdk

[![Crates.io](https://img.shields.io/crates/v/steam-webapi-rust-sdk.svg)](https://crates.io/crates/steam-webapi-rust-sdk)
[![docs.rs](https://img.shields.io/docsrs/steam-webapi-rust-sdk)](https://docs.rs/steam-webapi-rust-sdk/latest/steam_webapi_rust_sdk/)
[![License: MIT](https://img.shields.io/crates/l/steam-webapi-rust-sdk.svg)](LICENSE)

A Rust SDK for the [Steam Web API](https://steamcommunity.com/dev). It provides typed access to Steam
player, app, and Dota 2 data, with local response caching to keep you within Steam's rate limits.

## Features

- **App catalog** — list every app available on Steam (`ISteamApps/GetAppList`).
- **Store details** — retrieve detailed app metadata from the Steam store.
- **Player profiles** — profile summaries, friend lists, ban status, and vanity URL resolution via
  `ISteamUser`.
- **Owned games & activity** — owned games, recently played games, Steam level, and badges via
  `IPlayerService`.
- **Stats & achievements** — player achievements, game stats, stat/achievement schemas, global
  achievement percentages, and live player counts via `ISteamUserStats`.
- **App news** — news items for an app via `ISteamNews`.
- **Dota 2 data** — match history and details, heroes, league listings, live league games, and team
  info via `IDOTA2Match_570`.
- **Response caching** — API responses are cached locally under `steam-webapi-cache/`, minimizing
  redundant network calls.
- **Prebuilt cache** — a ready-made cache of app details is available for download so you don't have
  to fetch every app individually.
- **SteamID helpers** — convert account IDs between 32-bit and 64-bit representations.

## Installation

```
cargo add steam-webapi-rust-sdk
```

or add it to your `Cargo.toml`:

```toml
[dependencies]
steam-webapi-rust-sdk = "0.0.9"
```

## Configuration

The SDK needs a Steam Web API key. Grab one from the
[Steam API key registration page](https://steamcommunity.com/dev/apikey), then export it as an
environment variable:

```
export STEAM_WEBAPI_KEY="YOUR_STEAM_WEBAPI_KEY"
```

Add that line to your shell profile (`~/.bash_profile`, `~/.zshrc`, etc.) and reload it with
`source ~/.bash_profile` so it's available whenever you run your project.

## Usage

```rust
// List every app available on Steam
let app_list = steam_webapi_rust_sdk::get_app_list()?;

// Get store details for a given app id
let app_details = steam_webapi_rust_sdk::get_app_details(570)?;

// Query Dota 2 match history for an account
let match_history = steam_webapi_rust_sdk::get_dota2_match_history(
    Some(76561197960361544), // account_id
    None,                    // game_mode
    None,                    // skill
    None,                    // min_players
    None,                    // start_at_match_id
    None,                    // matches_requested
    None,                    // tournament_games_only
)?;

// Look up a player's profile summary
let summaries = steam_webapi_rust_sdk::get_player_summaries(vec![76561197960361544])?;

// Check the current number of players in-game (no API key required)
let player_count = steam_webapi_rust_sdk::get_number_of_current_players(570)?;

// Convert a 32-bit SteamID to its 64-bit form
let account_id_64 = steam_webapi_rust_sdk::convert_32bit_account_id_to_64bit(95816);
```

## Caching

Responses are written to a `steam-webapi-cache/` folder in your project root, and cached variants
(`get_cached_app_list`, `get_cached_app_details`) read from it first before falling back to the network.

To skip the initial download entirely, grab the prebuilt cache of all Steam app details from
[Google Drive](https://drive.google.com/drive/folders/1lpx0Bwzhc3ABEQp80lV1XiwOzONY9OYl?usp=sharing)
and extract it into your project root. A SHA-256 checksum is included so you can verify its integrity.

## Demo Applications

- [Retrieve details for all Steam apps](https://github.com/bohdaq/retrieve-all-steam-apps-details-demo-app)
- [List all properties in the app details API response](https://github.com/bohdaq/list-steam-appdetails-properties)
- [Retrieve Dota 2 match history](https://github.com/bohdaq/dota2-match-history-demo-app)

## Documentation

Full API reference is available on [docs.rs](https://docs.rs/steam-webapi-rust-sdk/latest/steam_webapi_rust_sdk/).

## Build

Requires [Rust](https://www.rust-lang.org/tools/install).

```
cargo build
```

## Test

Some tests call the live Steam API, so you'll need an internet connection and a valid
`STEAM_WEBAPI_KEY` set.

```
cargo test
```

## Contributing

Contributions are welcome — see [CONTRIBUTING.md](CONTRIBUTING.md) for the process.

## License

Licensed under the [MIT License](LICENSE).
