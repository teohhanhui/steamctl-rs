use bpaf::{construct, long, positional, OptionParser, Parser};
use effing_mad::effectful;
use secrecy::ExposeSecret;

use crate::effects::Console;

#[derive(Clone, Debug)]
pub enum AuthenticatorOptions {
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

pub fn authenticator() -> OptionParser<AuthenticatorOptions> {
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

fn add() -> impl Parser<AuthenticatorOptions> {
    let force = long("force")
        .help("Overwrite existing authenticator.")
        .switch();
    let from_secret = long("from-secret")
        .help("Provide the authenticator secret directly. Need to be base64 encoded.")
        .argument("FROM_SECRET")
        .optional();

    construct!(AuthenticatorOptions::Add {
        force,
        from_secret,
        // positionals
        account(),
    })
}

fn code() -> impl Parser<AuthenticatorOptions> {
    construct!(AuthenticatorOptions::Code {
        // positionals
        account(),
    })
}

fn qr_code() -> impl Parser<AuthenticatorOptions> {
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

    construct!(AuthenticatorOptions::QrCode {
        compat,
        invert,
        // positionals
        account(),
    })
}

fn account() -> impl Parser<String> {
    positional("account").help("Account name")
}

#[effectful(Console<'a>)]
pub fn process_authenticator_command<'a>(options: AuthenticatorOptions) {
    match options {
        AuthenticatorOptions::Add {
            account,
            force,
            from_secret,
        } => process_authenticator_add_command(account, force, from_secret).do_,
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
}

#[effectful(Console<'a>)]
fn process_authenticator_add_command<'a>(
    account: String,
    force: bool,
    from_secret: Option<String>,
) {
    // TODO: `force` and `from_secret`
    yield Console::println("To add an authenticator, first we need to login to Steam".into());
    yield Console::println(format!("Account name: {account}").into());
    yield Console::print(format!("Enter password for '{account}': ").into());
    yield Console::flush();
    let password = yield Console::read_line_hidden();
    // TODO: don't expose password
    yield Console::println(format!("password: {:?}", password.expose_secret()).into());
}
