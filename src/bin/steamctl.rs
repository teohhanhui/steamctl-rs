#![feature(coroutine_trait)]
#![feature(coroutines)]

use std::sync::OnceLock;

use anyhow::Result;
use effing_mad::{handle_group, handler};
use steamctl::{
    commands,
    effects::{
        _build_device_details,
        _transport,
        begin_auth_via_credentials,
        flush,
        print,
        println,
        read_line_hidden,
        read_options,
        Cli,
        Console,
        SteamWebApi,
        UserLogin,
    },
    handlers,
};
use steamguard::transport::WebApiTransport;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let transport: OnceLock<WebApiTransport> = OnceLock::new();

    let user_login_handler = handler!(UserLogin {
        _build_device_details() => handlers::user_login::_build_device_details(),
        begin_auth_via_credentials(_self, username, password) => {
            handlers::user_login::begin_auth_via_credentials(_self, username, password)
        },
    });
    let with_user_login = handle_group(commands::process(), user_login_handler);
    let steam_web_api_handler = handler!(SteamWebApi<'_> {
        _transport() => handlers::steam_web_api::_transport(&transport),
    });
    let with_steam_web_api = handle_group(with_user_login, steam_web_api_handler);
    let console_handler = handler!(Console<'_> {
        println(s) => handlers::console::println(s),
        print(s) => handlers::console::print(s),
        flush() => handlers::console::flush(),
        read_line_hidden() => handlers::console::read_line_hidden(),
    });
    let with_console = handle_group(with_steam_web_api, console_handler);
    let cli_handler = handler!(Cli {
        read_options() => handlers::cli::read_options(),
    });
    let with_cli = handle_group(with_console, cli_handler);

    effing_mad::run(with_cli)
}
