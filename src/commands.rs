use anyhow::Result;
use effing_mad::effectful;
use tracing::debug;

use crate::{
    cli_options::Options,
    effects::{Cli, Console, SteamWebApi},
};

mod authenticator;
mod common;
mod depot;

#[effectful(Cli, Console<'a>, SteamWebApi<'a>)]
pub fn process<'a>() -> Result<()> {
    let options = yield Cli::read_options();
    debug!(?options);
    match options {
        Options::Authenticator(authenticator_options) => {
            (self::authenticator::process(authenticator_options).do_)?
        },
        Options::Depot(depot_options) => {
            todo!()
        },
    }
    Ok(())
}
