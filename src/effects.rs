use std::{borrow::Cow, marker::PhantomData};

use effing_mad::{frunk::Coprod, Effect, EffectGroup};
use secrecy::SecretString;

use crate::cli_options::Options;

pub struct Console<'a>(PhantomData<&'a ()>);

#[allow(non_camel_case_types)]
pub struct println<'a>(pub Cow<'a, str>);

#[allow(non_camel_case_types)]
pub struct print<'a>(pub Cow<'a, str>);

#[allow(non_camel_case_types)]
pub struct flush();

#[allow(non_camel_case_types)]
pub struct read_line_hidden();

pub struct Cli();

#[allow(non_camel_case_types)]
pub struct read_options();

impl<'a> Console<'a> {
    pub fn println(s: Cow<'a, str>) -> println<'a> {
        println(s)
    }

    pub fn print(s: Cow<'a, str>) -> print<'a> {
        print(s)
    }

    pub fn flush() -> flush {
        flush()
    }

    pub fn read_line_hidden() -> read_line_hidden {
        read_line_hidden()
    }
}

impl<'a> EffectGroup for Console<'a> {
    type Effects = Coprod!(println<'a>, print<'a>, flush, read_line_hidden);
}

impl<'a> Effect for println<'a> {
    type Injection = ();
}

impl<'a> Effect for print<'a> {
    type Injection = ();
}

impl Effect for flush {
    type Injection = ();
}

impl Effect for read_line_hidden {
    type Injection = SecretString;
}

impl Cli {
    pub fn read_options() -> read_options {
        read_options()
    }
}

impl EffectGroup for Cli {
    type Effects = Coprod!(read_options);
}

impl Effect for read_options {
    type Injection = Options;
}
