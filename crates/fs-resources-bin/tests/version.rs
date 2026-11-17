//! The packaged binary says its version and its help without opening a window or a server (the start test of the release),
//! and an unknown option ends with 2 and a message.
//!
//! Tests: example-based, on the real binary.

use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_fs-resources"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn the_version_is_printed_and_nothing_is_started() {
    for flag in ["--version", "-V"] {
        let out = run(&[flag]);
        assert!(
            out.status.success(),
            "{flag}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&out.stdout).trim(),
            format!("fs-resources {}", env!("CARGO_PKG_VERSION"))
        );
    }
}

#[test]
fn the_help_is_printed_with_exit_code_0() {
    for flag in ["--help", "-h"] {
        let out = run(&[flag]);
        assert_eq!(out.status.code(), Some(0), "{flag}");
        let said = String::from_utf8_lossy(&out.stdout);
        assert!(
            said.contains("fs-resources serve") && said.contains("--frontend"),
            "{said}"
        );
    }
}

#[test]
fn an_unknown_option_exits_with_2_and_names_it() {
    let out = run(&["--nope"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("unknown option --nope"));
}

#[test]
fn serve_help_exits_with_0_and_starts_no_server() {
    let out = run(&["serve", "--help"]);
    assert_eq!(out.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&out.stdout).contains("resources.sock"));
}
