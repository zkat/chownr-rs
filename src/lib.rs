use std::fs;
use std::path::Path;

use failure::Fail;
use nix::unistd::{self, Uid, Gid};

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Io(#[fail(cause)] std::io::Error),
    #[fail(display = "{}", _0)]
    Nix(#[fail(cause)] nix::Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<nix::Error> for Error {
    fn from(error: nix::Error) -> Self {
        Error::Nix(error)
    }
}

pub fn chownr(path: &Path, uid: Option<Uid>, gid: Option<Gid>) -> Result<(), Error> {
    if path.is_dir() {
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            chownr(entry.path().as_path(), uid, gid)?;
        }
    }
    unistd::chown(path, uid, gid)?;
    Ok(())
}
