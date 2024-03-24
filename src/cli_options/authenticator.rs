use bpaf::{construct, long, positional, OptionParser, Parser};

#[derive(Clone, Debug)]
pub enum Authenticator {
    Add {
        account: String,
        force: bool,
        from_secret: Option<String>, // TODO: use a `Base64` newtype?
    },
    Code {
        account: String,
    },
    QrCode {
        account: String,
        compat: bool,
        invert: bool,
    },
}

pub fn authenticator() -> OptionParser<Authenticator> {
    let add = add()
        .to_options()
        .descr("Add authentictor to a Steam account")
        .command("add");
    let code = code()
        .to_options()
        .descr("Generate auth code")
        .command("code");
    let qr_code = qr_code()
        .to_options()
        .descr("Generate QR code")
        .command("qrcode");

    construct!([add, code, qr_code]).to_options()
}

fn add() -> impl Parser<Authenticator> {
    let force = long("force")
        .help("Overwrite existing authenticator.")
        .switch();
    let from_secret = long("from-secret")
        .help("Provide the authenticator secret directly. Need to be base64 encoded.")
        .argument("FROM_SECRET")
        .optional();

    construct!(Authenticator::Add {
        force,
        from_secret,
        // positionals
        account(),
    })
}

fn code() -> impl Parser<Authenticator> {
    construct!(Authenticator::Code {
        // positionals
        account(),
    })
}

fn qr_code() -> impl Parser<Authenticator> {
    let compat = long("compat")
        .help(
            "Alternative QR code mode (e.g. otpauth://totp/....). The default mode is custom \
             format (otpath://steam/...), and requires custom Steam support in your 2FA app. Apps \
             with support: Aegis and andOTP.",
        )
        .switch();
    let invert = long("invert")
        .help("Invert QR code colors. Try if app fails to scan the code.")
        .switch();

    construct!(Authenticator::QrCode {
        compat,
        invert,
        // positionals
        account(),
    })
}

fn account() -> impl Parser<String> {
    positional("account").help("Account name")
}
