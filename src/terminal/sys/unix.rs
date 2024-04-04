use std::{
    fs,
    io::{self, IsTerminal, Stdin},
    os::fd::{AsFd, AsRawFd},
};

use parking_lot::Mutex;
use termios::{tcsetattr, Termios, ECHO, ECHONL, TCSANOW};

use super::file_descriptor::FileDescriptor;

// Some(Termios) -> we're in the hidden input mode and this is the previous mode
// None -> we're not in the hidden input mode
static TERMINAL_MODE_PRIOR_HIDDEN_INPUT_MODE: Mutex<Option<Termios>> =
    parking_lot::const_mutex(None);

pub(crate) fn enable_hidden_input_mode() -> io::Result<()> {
    let mut original_mode = TERMINAL_MODE_PRIOR_HIDDEN_INPUT_MODE.lock();

    if original_mode.is_some() {
        return Ok(());
    }

    let stdin = io::stdin();
    let tty_fd = tty_fd(&stdin)?;
    let tty_raw_fd = tty_fd.as_raw_fd();

    let mut termios = Termios::from_fd(tty_raw_fd)?;
    let original_mode_termios = termios;

    // Hide the input
    termios.c_lflag &= !ECHO;
    // But don't hide the newline character when the user hits Enter
    termios.c_lflag |= ECHONL;

    tcsetattr(tty_raw_fd, TCSANOW, &termios)?;

    // Keep it last - set the original mode only if we were able to switch to the
    // hidden input mode
    *original_mode = Some(original_mode_termios);

    Ok(())
}

pub(crate) fn disable_hidden_input_mode() -> io::Result<()> {
    let mut original_mode = TERMINAL_MODE_PRIOR_HIDDEN_INPUT_MODE.lock();

    if let Some(original_mode_termios) = original_mode.as_ref() {
        let stdin = io::stdin();
        let tty_fd = tty_fd(&stdin)?;
        let tty_raw_fd = tty_fd.as_raw_fd();

        tcsetattr(tty_raw_fd, TCSANOW, original_mode_termios)?;

        // Keep it last - remove the original mode only if we were able to switch back
        *original_mode = None;
    }

    Ok(())
}

pub(crate) fn tty_fd(stdin: &Stdin) -> io::Result<FileDescriptor<'_>> {
    let tty_fd = if io::stdin().is_terminal() {
        FileDescriptor::Borrowed(stdin.as_fd())
    } else {
        let tty_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/tty")?;
        FileDescriptor::Owned(tty_file.into())
    };
    Ok(tty_fd)
}
