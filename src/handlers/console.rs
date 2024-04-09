use std::{
    borrow::Cow,
    io::{self, Write},
};

use anyhow::Result;
use secrecy::SecretString;

use crate::terminal::{disable_hidden_input_mode, enable_hidden_input_mode};

pub fn println(s: Cow<'_, str>) {
    println!("{s}");
}

pub fn print(s: Cow<'_, str>) {
    print!("{s}");
}

pub fn flush() -> Result<()> {
    io::stdout().flush()?;
    Ok(())
}

pub fn read_line_hidden() -> Result<SecretString> {
    let mut line = String::new();
    enable_hidden_input_mode()?;
    io::stdin().read_line(&mut line)?;
    disable_hidden_input_mode()?;
    let line = if let Some(line) = line.strip_suffix('\n') {
        line.to_owned()
    } else {
        line
    };
    Ok(line.into())
}
