use bpaf::{construct, OptionParser, Parser};

#[derive(Clone, Debug)]
pub enum Authenticator {
    Add {
        //
    },
    QrCode {
        //
    },
}

pub fn authenticator() -> OptionParser<Authenticator> {
    let add = add()
        .to_options()
        .descr("Add authentictor to a Steam account")
        .command("add");
    let qr_code = qr_code()
        .to_options()
        .descr("Generate QR code")
        .command("qrcode");

    construct!([add, qr_code]).to_options()
}

fn add() -> impl Parser<Authenticator> {
    //

    construct!(Authenticator::Add {})
}

fn qr_code() -> impl Parser<Authenticator> {
    //

    construct!(Authenticator::QrCode {})
}
