use super::LockedDirectory;
use std::fs::File;
use std::io::Read;
use sysinfo::Pid;

#[test]
fn test_pidlock_lifecycle() {
    let temp_dir = tempfile::tempdir().unwrap();
    let pidfile = LockedDirectory::new(temp_dir.path()).unwrap();
    let pidfile_path = temp_dir.path().join(LockedDirectory::LOCKFILE_NAME);
    assert!(pidfile_path.exists());

    let mut file = File::open(&pidfile_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    assert_eq!(contents, format!("{}", std::process::id()));

    drop(pidfile);
    assert!(!pidfile_path.exists());
}

#[test]
fn test_pidlock_locks() {
    let temp_dir = tempfile::tempdir().unwrap();
    let pidfile = LockedDirectory::new(temp_dir.path()).unwrap();
    LockedDirectory::with_pid(temp_dir.path(), Pid::from(0)).expect_err("pidfile already exists");
    drop(pidfile);
    LockedDirectory::with_pid(temp_dir.path(), Pid::from(0)).expect("other PID can take over");
}
