use anyhow::Result;
use effing_mad::effectful;

use super::common::login;
use crate::{
    cli_options::AuthenticatorOptions,
    effects::{Console, SteamWebApi},
};

#[effectful(Console<'a>, SteamWebApi<'a>)]
pub fn process<'a>(options: AuthenticatorOptions) -> Result<()> {
    match options {
        AuthenticatorOptions::Add {
            account,
            force,
            from_secret,
        } => (add(account, force, from_secret).do_)?,
        AuthenticatorOptions::Code { account } => {
            todo!()
        },
        AuthenticatorOptions::QrCode {
            account,
            compat,
            invert,
        } => {
            todo!()
        },
    }
    Ok(())
}

#[effectful(Console<'a>, SteamWebApi<'a>)]
pub fn add<'a>(account: String, force: bool, from_secret: Option<String>) -> Result<()> {
    // TODO: `force` and `from_secret`
    yield Console::println("To add an authenticator, first we need to login to Steam".into());
    yield Console::println(format!("Account name: {account}").into());
    (login(account, None).do_)?;
    todo!()
}
