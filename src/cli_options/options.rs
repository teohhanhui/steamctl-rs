use bpaf::{construct, OptionParser, Parser};
use effing_mad::effectful;

use super::{
    authenticator::{authenticator, process_authenticator_command, AuthenticatorOptions},
    depot::{depot, DepotOptions},
};
use crate::effects::{CliOptions, Console};

#[derive(Clone, Debug)]
pub enum Options {
    Authenticator(AuthenticatorOptions),
    Depot(DepotOptions),
}

pub fn options() -> OptionParser<Options> {
    let authenticator = authenticator()
        .descr("Manage Steam authenticators")
        .command("authenticator");
    let authenticator = construct!(Options::Authenticator(authenticator));
    let depot = depot()
        .descr("List and download from Steam depots")
        .command("depot");
    let depot = construct!(Options::Depot(depot));

    construct!([authenticator, depot]).to_options()
}

#[effectful(CliOptions, Console<'a>)]
pub fn process_command<'a>() {
    let options = yield CliOptions::read();
    yield Console::println(format!("{options:?}").into()); // TODO: this should be `Log::debug`?
    match options {
        Options::Authenticator(authenticator_options) => {
            process_authenticator_command(authenticator_options).do_
        },
        Options::Depot(depot_options) => {
            todo!()
        },
    }
}
