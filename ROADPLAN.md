# Roadmap

Status snapshot as of v0.0.9: the SDK covers `ISteamApps.GetAppList`, `store.steampowered.com/api/appdetails`,
all of `IDOTA2Match_570` (`GetMatchHistory`, `GetMatchDetails`, `GetHeroes`, `GetLeagueListing`,
`GetLiveLeagueGames`, `GetTeamInfoByTeamID`), `ISteamUser` (`GetPlayerSummaries`, `GetFriendList`,
`GetPlayerBans`, `ResolveVanityURL`), `IPlayerService` (`GetOwnedGames`, `GetRecentlyPlayedGames`,
`GetSteamLevel`, `GetBadges`), `ISteamUserStats` (`GetPlayerAchievements`, `GetUserStatsForGame`,
`GetSchemaForGame`, `GetGlobalAchievementPercentagesForApp`, `GetNumberOfCurrentPlayers`), and `ISteamNews`
(`GetNewsForApp`). App-details/match-details responses are cached locally under `steam-webapi-cache/`.
Account id 32/64-bit conversion helpers exist. This plan tracks what's next, roughly in priority order.

## Phase 1 — Finish what's already started

- [x] **Finish `GetMatchDetails`** (`src/idota2match_570/get_match_details/mod.rs`): `parse_response` now reads
      every field off `json["result"]` (including a `players` array), returns an `Err` when the API responds
      with `result.error` (e.g. an unknown match id), and added a `get()` that calls the API and caches the
      response, mirroring `get_match_history`/`appdetails`.
- [x] Wired `get_dota2_match_details` / `get_cached_dota2_match_details` through `idota2match_570::mod.rs` and
      exposed both as `steam_webapi_rust_sdk::get_dota2_match_details` / `get_cached_dota2_match_details` in
      `lib.rs`, matching the `get_app_details` / `get_cached_app_details` pattern.
- [x] Added unit tests in `src/idota2match_570/get_match_details/tests.rs` (API URL building, cache filepath,
      successful parse against the existing `test/idota2match_570/get_match_details/1461414523.json` fixture,
      and the two error paths) plus doctest examples in `lib.rs`.

## Phase 2 — Broaden Steam Web API coverage

- [x] `ISteamUser`: `GetPlayerSummaries`, `GetFriendList`, `GetPlayerBans`, `ResolveVanityURL`
      (`src/isteam_user/`).
- [x] `IPlayerService`: `GetOwnedGames`, `GetRecentlyPlayedGames`, `GetSteamLevel`, `GetBadges`
      (`src/iplayer_service/`).
- [x] `ISteamUserStats`: `GetPlayerAchievements`, `GetUserStatsForGame`, `GetSchemaForGame`,
      `GetGlobalAchievementPercentagesForApp`, `GetNumberOfCurrentPlayers` (`src/isteam_user_stats/`; the
      latter two endpoints work without a key, noted in their doc comments).
- [x] `IDOTA2Match_570` extras: `GetHeroes`, `GetLeagueListing`, `GetLiveLeagueGames`, `GetTeamInfoByTeamID`
      (added under the existing `src/idota2match_570/`).
- [x] `ISteamNews`: `GetNewsForApp` (`src/isteam_news/`).

Each new interface follows the existing module shape: `src/<interface>.rs` with `get_interface()`, one
submodule per method with `get_api_url`, `parse_response`, `get()`, and a `tests.rs` with unit tests (API URL
building + parsing against inline JSON fixtures matching Steam's documented response shapes — not live-API
tests, to keep this batch of ~18 new endpoints fast and offline-testable; see Phase 5 for the existing
fixture-based modules' live-API test philosophy). All 18 endpoints are wired through to `lib.rs` with brief
doc comments; their doctest examples are `no_run` since they need a real `STEAM_WEBAPI_KEY` and live SteamIDs
to execute meaningfully. Shared tolerant JSON field extraction (`json_u64`/`json_i64`/`json_f64`/`json_str`/
`json_bool`) and a `build_steam_api_url` helper were added to `util` to avoid repeating that boilerplate
across the new modules.

## Phase 3 — Further API surface & convenience features

Lower priority than Phase 2, roughly in order of expected payoff:

- [ ] `store.steampowered.com/api/appreviews` — review text/scores with cursor-based pagination; pairs
      naturally with the existing `appdetails` module.
- [ ] Currency/region-aware pricing on `appdetails` (the `cc` / `l` query params) — small addition to the
      existing module, not a new interface.
- [ ] `ISteamWebAPIUtil.GetSupportedAPIList` — could double as a coverage report generator: diff what
      Steam exposes against what this SDK implements.
- [ ] Auto-batching helper for endpoints capped at 100 IDs/call (e.g. `GetPlayerSummaries`) so callers
      don't have to chunk SteamID lists themselves.
- [ ] "Profile snapshot" convenience function combining `GetPlayerSummaries` + `GetOwnedGames` +
      achievements into one struct/call, on top of the individual endpoint functions.
- [ ] Workshop support (`IPublishedFileService.QueryFiles`, `GetPublishedFileDetails`) — only if there's
      demand from mod-tooling use cases.
- [ ] Other game item/econ interfaces (TF2 `IEconItems_440`, CS2 `ICSGOServers_730`) — same shape as the
      existing Dota2 module, worth doing if a user asks for a specific game.
- [ ] SteamSpy (ownership estimates, tags) as an optional non-Valve data source — keep behind a feature
      flag or separate module since it's a third-party API, not official Steam/Valve.

## Phase 4 — Core SDK quality

- [ ] Replace `String` errors with a proper `enum SteamApiError` (or `thiserror`) so callers can match on
      failure kind instead of comparing error strings (see the comment in `lib.rs::get_app_details` doc
      example — it already string-compares error messages, which is brittle).
- [ ] Add retry/backoff for transient network failures in `make_api_call` (`src/lib.rs`), since Steam rate-limits
      at ~200 req/5min per IP and currently a single failure just bubbles up.
- [ ] Make the cache directory configurable (currently hardcoded `"steam-webapi-cache"` in
      `util::get_cache_dir_path`) via an optional env var or parameter, for multi-project setups.
- [ ] Consider an async API (behind a feature flag, e.g. `reqwest` + `tokio`) alongside the current
      blocking `minreq` calls, since most Rust web backends are async today.

## Phase 5 — Testing & CI

- [ ] Add a GitHub Actions workflow: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
      on push/PR. None currently exists in the repo.
- [ ] Reduce live-API dependency in tests where practical (e.g. record/replay fixtures for parsing logic,
      keeping a smaller set of true integration tests that need `STEAM_WEBAPI_KEY` + network).
- [ ] Add `#[deny(missing_docs)]` or doc coverage check so every new public function ships doctest examples,
      matching the existing convention in `lib.rs`.

## Phase 6 — Docs & release hygiene

- [x] Update README feature list as each new interface lands.
- [x] Keep `docs.rs` link in README pinned to latest published version — now links to `/latest/` instead of
      a hardcoded version, so it never goes stale again.
- [ ] Tag releases consistently with `Cargo.toml` version bumps (existing tags go up to `0.0.8`; this work
      bumps to `0.0.9` but the corresponding git tag/crates.io publish is a separate manual step).

---
Revisit this file each time a phase item lands or priorities shift — treat it as a living backlog, not a fixed spec.
