use anyhow::Result;
use effing_mad::effectful;
use secrecy::{ExposeSecret, SecretString};
use steamguard::{
    protobufs::steammessages_auth_steamclient::EAuthTokenPlatformType,
    token::Tokens,
    DeviceDetails,
    LoginError,
    UserLogin,
};
use tracing::{debug, error, info};

use crate::effects::{Console, SteamWebApi};

#[effectful(Console<'a>, SteamWebApi<'a>)]
pub fn login<'a>(username: String, password: Option<SecretString>) -> Result<Tokens> {
    let transport = (yield SteamWebApi::_transport())?;

    let mut login = UserLogin::new(transport.clone(), build_device_details());

    let mut password = if let Some(p) = password {
        p
    } else {
        (prompt_password(format!("Enter password for {username}: ")).do_)?
    };
    let confirmation_methods;
    loop {
        // TODO: this is obviously an I/O effect... but I have tried and failed to
        // appease the compiler
        match login.begin_auth_via_credentials(&username, password.expose_secret()) {
            Ok(methods) => {
                confirmation_methods = methods;
                break;
            },
            Err(LoginError::TooManyAttempts) => {
                error!(
                    "Too many login attempts. Steam is rate limiting you. Please wait a while and \
                     try again later."
                );
                return Err(LoginError::TooManyAttempts.into());
            },
            Err(LoginError::BadCredentials) => {
                info!("Incorrect password.");
                password = (prompt_password(format!(
                    "Invalid password for {username}. Enter password: "
                ))
                .do_)?;
                continue;
            },
            Err(err) => {
                error!(%err, "Unexpected error when trying to log in.");
                return Err(err.into());
            },
        }
    }
    debug!(?confirmation_methods);
    todo!()
}

#[effectful(Console<'a>)]
fn prompt_password<'a>(prompt: String) -> Result<SecretString> {
    yield Console::print(prompt.into());
    (yield Console::flush())?;
    yield Console::read_line_hidden()
}

fn build_device_details() -> DeviceDetails {
    DeviceDetails {
        friendly_name: format!(
            "{} (steamctl)",
            gethostname::gethostname()
                .into_string()
                .expect("failed to get hostname")
        ),
        platform_type: EAuthTokenPlatformType::k_EAuthTokenPlatformType_MobileApp,
        os_type: -500,
        gaming_device_type: 528,
    }
}
