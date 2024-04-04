#![feature(coroutine_trait)]
#![feature(coroutines)]

use std::io::{self, Write};

use effing_mad::{handle_group, handler};
use steamctl::{
    cli_options::{options, process_command},
    effects::{flush, print, println, read_line_hidden, read_options, Cli, Console},
    terminal::{disable_hidden_input_mode, enable_hidden_input_mode},
};

fn main() {
    let console_handler = handler!(Console {
        println(s) => println!("{s}"),
        print(s) => print!("{s}"),
        flush() => {
            if io::stdout().flush().is_err() {
                // return ControlFlow::Break(());
                break;
            }
        },
        read_line_hidden() => {
            let mut line = String::new();
            if enable_hidden_input_mode().is_err() {
                // return ControlFlow::Break(());
                break;
            }
            if io::stdin().read_line(&mut line).is_err() {
                // return ControlFlow::Break(());
                break;
            }
            if disable_hidden_input_mode().is_err() {
                // return ControlFlow::Break(());
                break;
            }
            let line = if let Some(line) = line.strip_suffix('\n') {
                line.to_owned()
            } else {
                line
            };
            line.into()
        },
    });
    let with_console = handle_group(process_command(), console_handler);
    let cli_handler = handler!(Cli {
        read_options() => options().run(),
    });
    let with_cli = handle_group(with_console, cli_handler);

    effing_mad::run(with_cli);
}
