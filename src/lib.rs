use std::fs;
use std::path::Path;

use nix::unistd::{self, Uid, Gid};

pub mod errors {
    use error_chain::error_chain;
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Nix(nix::Error);
        }
    }
}

pub fn chownr(path: &Path, uid: Option<Uid>, gid: Option<Gid>) -> errors::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            chownr(entry.path().as_path(), uid, gid)?;
        }
    }
    unistd::chown(path, uid, gid)?;
    Ok(())
}
