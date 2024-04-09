#![feature(coroutine_trait)]
#![feature(coroutines)]

use std::cell::OnceCell;

use anyhow::Result;
use effing_mad::{handle_group, handler};
use steamctl::{
    commands,
    effects::{
        flush,
        print,
        println,
        read_line_hidden,
        read_options,
        Cli,
        Console,
        SteamWebApi,
        _transport,
    },
    handlers,
};
use steamguard::transport::WebApiTransport;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let transport: OnceCell<WebApiTransport> = OnceCell::new();
    let steam_web_api_handler = handler!(SteamWebApi<'_> {
        _transport() => handlers::steam_web_api::_transport(&transport),
    });
    let with_steam_web_api = handle_group(commands::process(), steam_web_api_handler);
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
