use std::borrow::Cow;

use anyhow::Result;
use effing_mad::effects;
use secrecy::SecretString;
use steamguard::transport::WebApiTransport;

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
        pub fn flush() -> Result<()>;
        pub fn read_line_hidden() -> Result<SecretString>;
    }
}

effects! {
    pub State<T> {
        pub fn get() -> T;
        pub fn put(v: T) -> ();
    }
}

effects! {
    pub SteamWebApi<'a> {
        pub fn _transport() -> Result<&'a WebApiTransport>;
    }
}
