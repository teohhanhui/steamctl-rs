use std::io;

pub(crate) mod sys;

pub fn enable_hidden_input_mode() -> io::Result<()> {
    sys::enable_hidden_input_mode()
}

pub fn disable_hidden_input_mode() -> io::Result<()> {
    sys::disable_hidden_input_mode()
}
