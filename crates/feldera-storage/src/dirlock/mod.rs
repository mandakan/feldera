//! A simple PID-based locking mechanism.
//!
//! Makes sure we don't accidentally run multiple instances of the program
//! using the same data directory.

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};
use sysinfo::{Pid, System};

#[cfg(test)]
mod test;

/// Check whether a process exists, used to determine whether a pid file is
/// stale.
fn process_exists(pid: u32) -> bool {
    let s = System::new_all();
    s.process(Pid::from(pid as usize)).is_some()
}

/// An instance of a PID file.
///
/// The PID file is removed from the FS when the instance is dropped.
#[derive(Debug)]
pub struct LockedDirectory {
    base: PathBuf,
    pid: Pid,
}

impl Drop for LockedDirectory {
    fn drop(&mut self) {
        let pid_file = self.base.join(LockedDirectory::LOCKFILE_NAME);
        if pid_file.exists() {
            log::trace!("Removing pidfile: {}", pid_file.display());
            std::fs::remove_file(&pid_file).unwrap();
        }
    }
}

impl LockedDirectory {
    const LOCKFILE_NAME: &'static str = "feldera.pidlock";

    fn with_pid<P: AsRef<Path>>(base_path: P, pid: Pid) -> Result<LockedDirectory, String> {
        let pid_str = pid.to_string();
        let pid_file = base_path.as_ref().join(LockedDirectory::LOCKFILE_NAME);
        if pid_file.exists() {
            let mut file = File::open(&pid_file).map_err(|e| e.to_string())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|e| e.to_string())?;
            let old_pid = contents.trim().parse::<u32>().map_err(|e| e.to_string())?;
            if process_exists(old_pid) {
                return Err(format!("pidfile already exists with pid {}", old_pid));
            } else if old_pid == pid.as_u32() {
                // The pidfile is ours, just leave it as is.
            } else {
                // The process doesn't exist, so we can safely overwrite the pidfile.
                log::debug!("Removing stale pidfile: {}", pid_file.display());
                std::fs::remove_file(&pid_file).map_err(|e| e.to_string())?;
            }
        }

        let mut file = File::create(&pid_file).map_err(|e| e.to_string())?;
        file.write_all(pid_str.as_bytes())
            .map_err(|e| e.to_string())?;

        Ok(LockedDirectory {
            base: base_path.as_ref().into(),
            pid,
        })
    }

    /// Attempts to create a new pidfile in the `base_path` directory,
    /// returning an error if the file was already created by a different
    /// process (and that process is still alive).
    ///
    /// # Panics
    /// - If the current process's PID cannot be determined.
    /// - If the `base_path` does not exist or is not a directory.
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<LockedDirectory, String> {
        let pid = sysinfo::get_current_pid().expect("failed to get current pid");
        assert!(base_path.as_ref().is_dir());
        Self::with_pid(base_path, pid)
    }

    /// Returns the PID of the process that created the pidfile.
    pub fn pid(&self) -> Pid {
        self.pid
    }

    /// Returns the path to the directory in which the pidfile was created.
    pub fn base(&self) -> &Path {
        self.base.as_path()
    }
}
