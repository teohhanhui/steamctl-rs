use std::sync::Arc;

use anyhow::Result;
use effing_mad::effectful;
use parking_lot::RwLock;
use secrecy::SecretString;
use steamguard::{token::Tokens, LoginError};
use tracing::{debug, error, info};

use crate::effects::{Console, SteamWebApi, UserLogin};

#[effectful(Console<'a>, SteamWebApi<'a>, UserLogin)]
pub fn login<'a>(username: String, password: Option<SecretString>) -> Result<Tokens> {
    let transport = (yield SteamWebApi::_transport())?;

    let login = Arc::new(RwLock::new(steamguard::UserLogin::new(
        transport.clone(),
        yield UserLogin::_build_device_details(),
    )));

    let mut password = if let Some(p) = password {
        p
    } else {
        (prompt_password(format!("Enter password for {username}: ")).do_)?
    };
    let confirmation_methods;
    loop {
        match yield UserLogin::begin_auth_via_credentials(
            Arc::clone(&login),
            username.clone(),
            password,
        ) {
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
