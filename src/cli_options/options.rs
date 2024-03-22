use bpaf::{construct, OptionParser, Parser};

use super::{
    authenticator::{authenticator, Authenticator},
    depot::{depot, Depot},
};

#[derive(Clone, Debug)]
pub enum Options {
    Authenticator(Authenticator),
    Depot(Depot),
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
