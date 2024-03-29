#![feature(coroutines)]
#![feature(coroutine_trait)]

use std::borrow::Cow;

use effing_mad::{effectful, handle, handle_group, handler, Effect};
use steamctl::cli_options::{options, Options};

struct ConsoleOutput<'a>(Cow<'a, str>);

impl<'a> Effect for ConsoleOutput<'a> {
    type Injection = ();
}

effing_mad::effects! {
    CliOptions {
        fn read() -> Options;
    }
}

fn main() {
    let console_handler = handler!(ConsoleOutput(s) => println!("{s}"));
    let with_console = handle(do_command(), console_handler);
    let cli_options_handler = handler!(CliOptions {
        read() => options().run(),
    });
    let with_cli_options = handle_group(with_console, cli_options_handler);

    effing_mad::run(with_cli_options);
}

#[effectful(CliOptions, ConsoleOutput<'a>)]
fn do_command<'a>() {
    let options = yield CliOptions::read();
    yield ConsoleOutput(format!("{options:?}").into());
}
