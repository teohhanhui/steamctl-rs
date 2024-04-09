use std::borrow::Cow;

use effing_mad::effects;
use secrecy::SecretString;

use crate::cli_options::Options;

effects! {
    pub Cli {
        pub fn read_options() -> Options;
    }
}

effects! {
    pub Console<'a> {
        pub fn println(pub s: Cow<'a, str>) -> ();
        pub fn print(pub s: Cow<'a, str>) -> ();
        pub fn flush() -> ();
        pub fn read_line_hidden() -> SecretString;
    }
}
