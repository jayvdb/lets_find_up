use std::path::PathBuf;

use lets_find_up::{FindUpKind, FindUpOptions};

#[test]
fn basic_find_up_file() {
    let left = lets_find_up::find_up("Cargo.toml").unwrap().unwrap();

    let right = path::resolve!(".", "Cargo.toml");
    assert_eq!(left, PathBuf::from(right));
}

#[test]
fn basic_find_up_dir() {
    let left = lets_find_up::find_up_with(
        "src",
        FindUpOptions {
            kind: FindUpKind::Dir,
            ..Default::default()
        },
    )
    .unwrap()
    .unwrap();

    let right = path::resolve!(".", "src");
    assert_eq!(left, PathBuf::from(right));
}
