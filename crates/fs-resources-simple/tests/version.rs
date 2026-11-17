//! The frontend binary says its version and its help, and an unknown option ends with 2.
//!
//! Tests: example-based, on the real binary.

use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_fs-resources-simple"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn the_version_is_printed_with_exit_code_0() {
    let out = run(&["--version"]);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout).trim(),
        format!("fs-resources-simple {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn the_help_exits_with_0_and_an_unknown_option_with_2() {
    assert_eq!(run(&["--help"]).status.code(), Some(0));
    let bad = run(&["--wrong"]);
    assert_eq!(bad.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&bad.stderr).contains("unknown option --wrong"));
}
