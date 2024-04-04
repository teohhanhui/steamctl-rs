#[cfg(unix)]
pub(crate) use self::unix::{disable_hidden_input_mode, enable_hidden_input_mode};

#[cfg(unix)]
mod file_descriptor;
#[cfg(unix)]
mod unix;
