use std::os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd, RawFd};

#[derive(Debug)]
pub(crate) enum FileDescriptor<'fd> {
    Owned(OwnedFd),
    Borrowed(BorrowedFd<'fd>),
}

impl From<OwnedFd> for FileDescriptor<'_> {
    fn from(fd: OwnedFd) -> Self {
        Self::Owned(fd)
    }
}

impl<'fd> From<BorrowedFd<'fd>> for FileDescriptor<'fd> {
    fn from(fd: BorrowedFd<'fd>) -> Self {
        Self::Borrowed(fd)
    }
}

impl AsFd for FileDescriptor<'_> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        match self {
            Self::Owned(fd) => fd.as_fd(),
            Self::Borrowed(fd) => *fd,
        }
    }
}

impl AsRawFd for FileDescriptor<'_> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        match self {
            Self::Owned(fd) => fd.as_raw_fd(),
            Self::Borrowed(fd) => fd.as_raw_fd(),
        }
    }
}
