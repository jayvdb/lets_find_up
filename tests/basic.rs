use std::path::{Path, PathBuf};
use sugar_path::SugarPath;

use lets_find_up::{FindUpKind, FindUpOptions};

#[test]
fn basic_find_in_current_directory() {
    let left = lets_find_up::find_up("Cargo.toml").unwrap().unwrap();

    let right = Path::new("Cargo.toml").absolutize();
    assert_eq!(left, PathBuf::from(right));
}

#[test]
fn basic_find_fails_when_missing() {
    let left = lets_find_up::find_up("file_that_doesnt_exist").unwrap();

    assert_eq!(left, None);
}

#[test]
fn find_in_current_directory() {
    let left = lets_find_up::find_up_with(
        "src",
        FindUpOptions {
            kind: FindUpKind::Dir,
            ..Default::default()
        },
    )
    .unwrap()
    .unwrap();

    let right = Path::new("src").absolutize();
    assert_eq!(left, PathBuf::from(right));
}

#[test]
fn find_in_parent_directory() {
    let mut start_at = std::env::current_dir().unwrap();
    start_at.push("src");

    let left = lets_find_up::find_up_with(
        "Cargo.lock",
        FindUpOptions {
            kind: FindUpKind::Dir,
            cwd: Path::new(&start_at),
        },
    )
    .unwrap()
    .unwrap();

    let right = Path::new("Cargo.lock").absolutize();
    assert_eq!(left, PathBuf::from(right));
}

#[test]
fn find_fails_when_missing_in_empty_directory() {
    let empty_dir = tempfile::TempDir::new().unwrap();

    let left = lets_find_up::find_up_with(
        "file_that_doesnt_exist",
        FindUpOptions {
            kind: FindUpKind::File,
            cwd: Path::new(&empty_dir.path()),
        },
    )
    .unwrap();

    assert_eq!(left, None);
}

#[test]
fn find_in_empty_subdirectory_directory() {
    let cwd = std::env::current_dir().unwrap();
    let sub_dir = tempfile::Builder::new().tempdir_in(cwd).unwrap();
    let sub_dir = sub_dir.path();
    eprintln!("sub_dir: {:?}", sub_dir);

    let left = lets_find_up::find_up_with(
        "Cargo.lock",
        FindUpOptions {
            kind: FindUpKind::File,
            cwd: sub_dir,
        },
    )
    .unwrap()
    .unwrap();

    let right = Path::new("Cargo.lock").absolutize();
    assert_eq!(left, PathBuf::from(right));
}
