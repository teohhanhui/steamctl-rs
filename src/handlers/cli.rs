use crate::cli_options::{options, Options};

pub fn read_options() -> Options {
    options().run()
}
