#![feature(coroutine_trait)]
#![feature(coroutines)]

use std::{
    io::{self, stdout, Write},
    ops::ControlFlow,
};

use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use effing_mad::{handle_group, handler};
use steamctl::{
    cli_options::{options, process_command},
    effects::*,
};

fn main() {
    let console_handler = handler!(Console {
        println(s) => println!("{s}"),
        print(s) => {
            print!("{s}");
            if io::stdout().flush().is_err() {
                // return ControlFlow::Break(());
                break;
            }
        },
        read_hidden() => {
            let mut s = String::new();
            let mut done_reading = false;
            if enable_raw_mode().is_err() {
                // return ControlFlow::Break(());
                break;
            }
            while !done_reading {
                let Ok(event) = read() else {
                    return ControlFlow::Break(());
                };
                match event {
                    Event::Key(KeyEvent { code: KeyCode::Char(c), .. }) => {
                        s.push(c);
                    },
                    Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => {
                        if disable_raw_mode().is_err() {
                            // return ControlFlow::Break(());
                            break;
                        }
                        let mut stdout = stdout();
                        if execute!(stdout, Print("\n")).is_err() {
                            // return ControlFlow::Break(());
                            break;
                        }
                        done_reading = true;
                    },
                    _ => continue,
                };
            }
            s.into()
        },
    });
    let with_console = handle_group(process_command(), console_handler);
    let cli_handler = handler!(Cli {
        read_options() => options().run(),
    });
    let with_cli = handle_group(with_console, cli_handler);

    effing_mad::run(with_cli);
}
