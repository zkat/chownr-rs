use std::fs;
use std::path::Path;

use anyhow::Result;
use nix::unistd::{self, Gid, Uid};

pub fn chownr(path: &Path, uid: Option<Uid>, gid: Option<Gid>) -> Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            chownr(entry.path().as_path(), uid, gid)?;
        }
    }
    unistd::chown(path, uid, gid)?;
    Ok(())
}
