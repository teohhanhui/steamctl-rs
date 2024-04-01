#![feature(coroutine_trait)]
#![feature(coroutines)]

use effing_mad::{handle_group, handler};
use steamctl::{
    cli_options::{options, process_command},
    effects::*,
};

fn main() {
    let console_handler = handler!(Console {
        println(s) => println!("{s}"),
        print(s) => print!("{s}"),
    });
    let with_console = handle_group(process_command(), console_handler);
    let cli_options_handler = handler!(CliOptions {
        read() => options().run(),
    });
    let with_cli_options = handle_group(with_console, cli_options_handler);

    effing_mad::run(with_cli_options);
}
