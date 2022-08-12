# Welcome to steam-webapi-rust-sdk!
Hi, `steam-webapi-rust-sdk` is a set of utility functions to access Steam Web API.

In order to use this library make sure to set STEAM_WEB_API_KEY system environment variable.

The library itself tries to minimize number of networks calls through the caching relevant 
responses to the 'steam-webapi-cache' folder.

There is already prebuilt cache for all steam apps, in order to use it, 
simply clone [steam-webapi-cache](https://github.com/bohdaq/steam-webapi-cache) 
into the root folder of your project.

## Features
1. Ability to get list of apps available on the Steam.
2. Ability to get detailed app description from Steam.
3. Local cache of the app details.


## Configuration
In order to use this library make sure to set STEAM_WEB_API_KEY system environment variable.

> $ vim ~/.bash_profile 

> export STEAM_WEBAPI_KEY="YOUR_STEAM_WEBAPI_KEY"

> $ source  ~/.bash_profile

## Cache
There is already prebuilt cache for all steam apps, in order to use it,
simply clone [steam-webapi-cache](https://github.com/bohdaq/steam-webapi-cache)
into the root folder of your project.

> $ cd steam-webapi-rust-sdk

> $ git clone git@github.com:bohdaq/steam-webapi-cache.git

## Demo
Simple [demo app to retrieve details for all apps](https://github.com/bohdaq/retrieve-all-steam-apps-details-demo-app) 
available on Steam store.

## Documentation
Public functions definitions and usage can be found at [cargo docs](https://docs.rs/steam-webapi-rust-sdk/0.0.1/steam_webapi_rust_sdk/).


## Build
If you want to build steam-webapi-rust-sdk on your own, make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

> $ cargo build 
 

## Test
If you want to test steam-webapi-rust-sdk. You need to be connected to the internet as some of the tests invoke Steam API.

> $ cargo test


## Community
Contact me on [Discord](https://discordapp.com/users/952173191659393025/) where you can ask questions and share ideas. Follow the [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Donations
If you appreciate my work and want to support it, feel free to do it via [PayPal](https://www.paypal.com/donate/?hosted_button_id=7J69SYZWSP6HJ).

