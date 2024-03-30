#![feature(coroutines)]
#![feature(coroutine_trait)]

use std::borrow::Cow;

use effing_mad::{effectful, handle_group, handler};
use steamctl::cli_options::{options, Options};

effing_mad::effects! {
    Console<'a> {
        fn print(s: Cow<'a, str>) -> ();
    }
}

effing_mad::effects! {
    CliOptions {
        fn read() -> Options;
    }
}

fn main() {
    let console_handler = handler!(Console {
        print(s, _) => println!("{s}"),
    });
    let with_console = handle_group(do_command(), console_handler);
    let cli_options_handler = handler!(CliOptions {
        read() => options().run(),
    });
    let with_cli_options = handle_group(with_console, cli_options_handler);

    effing_mad::run(with_cli_options);
}

#[effectful(CliOptions, Console<'a>)]
fn do_command<'a>() {
    let options = yield CliOptions::read();
    yield Console::print(format!("{options:?}").into());
}
