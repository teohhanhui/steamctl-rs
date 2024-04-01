use std::{borrow::Cow, marker::PhantomData};

use effing_mad::{frunk::Coprod, Effect, EffectGroup};

use crate::cli_options::Options;

pub struct Console<'a>(PhantomData<&'a ()>);

#[allow(non_camel_case_types)]
pub struct println<'a>(pub Cow<'a, str>);

#[allow(non_camel_case_types)]
pub struct print<'a>(pub Cow<'a, str>);

pub struct CliOptions();

#[allow(non_camel_case_types)]
pub struct read();

impl<'a> Console<'a> {
    pub fn println(s: Cow<'a, str>) -> println<'a> {
        println(s)
    }

    pub fn print(s: Cow<'a, str>) -> print<'a> {
        print(s)
    }
}

impl<'a> EffectGroup for Console<'a> {
    type Effects = Coprod!(println<'a>, print<'a>);
}

impl<'a> Effect for println<'a> {
    type Injection = ();
}

impl<'a> Effect for print<'a> {
    type Injection = ();
}

impl CliOptions {
    pub fn read() -> read {
        read()
    }
}

impl EffectGroup for CliOptions {
    type Effects = Coprod!(read);
}

impl Effect for read {
    type Injection = Options;
}
