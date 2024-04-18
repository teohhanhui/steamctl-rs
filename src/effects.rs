use std::{borrow::Cow, sync::Arc};

use anyhow::Result;
use effing_mad::effects;
use parking_lot::RwLock;
use secrecy::SecretString;
use steamguard::{transport::WebApiTransport, DeviceDetails};

use crate::{api_responses::AllowedConfirmation, cli_options::Options};

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

effects! {
    pub UserLogin {
        pub fn _build_device_details() -> DeviceDetails;
        pub fn begin_auth_via_credentials(
            pub _self: Arc<RwLock<steamguard::UserLogin<WebApiTransport>>>,
            pub username: String,
            pub password: SecretString,
        ) -> Result<Vec<AllowedConfirmation>, steamguard::LoginError>;
    }
}
