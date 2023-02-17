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
