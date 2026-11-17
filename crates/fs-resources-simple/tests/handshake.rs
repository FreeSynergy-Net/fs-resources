//! The handshake with a wrong contract version: exit code 3 and a message, never silent; a right one passes (ADR 0094 draft, item 8).
//!
//! Tests: example-based, a real server in a thread, the real binary as a process.

use std::path::Path;
use std::process::Command;
use std::thread::JoinHandle;
use std::time::Duration;

// `Control.stop` ends every server of this process (fs-rpc keeps one stop flag), so tests that start a server take this lock.
static ONE_SERVER_AT_A_TIME: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn start(socket: &Path, contract: &'static str) -> JoinHandle<()> {
    let socket = socket.to_owned();
    std::thread::spawn(move || fs_resources_server::run_on(&socket, contract).unwrap())
}

fn wait_for(socket: &Path) {
    for _ in 0..200 {
        if socket.exists() {
            return;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    panic!("the server did not bind");
}

fn check(socket: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_fs-resources-simple"))
        .arg("--check")
        .arg("--socket")
        .arg(socket)
        .output()
        .unwrap()
}

fn with_server(contract: &'static str, then: impl FnOnce(&Path)) {
    let _one = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let dir = tempfile::tempdir().unwrap();
    let socket = dir.path().join("resources.sock");
    let server = start(&socket, contract);
    wait_for(&socket);
    then(&socket);
    fs_resources_client::stop(&socket).unwrap();
    server.join().unwrap();
}

#[test]
fn a_server_with_a_wrong_contract_ends_the_check_with_3_and_a_message() {
    with_server("9.0", |socket| {
        let out = check(socket);
        assert_eq!(out.status.code(), Some(3));
        let said = String::from_utf8_lossy(&out.stderr);
        assert!(
            said.contains("9.0") && said.contains("this frontend needs"),
            "{said}"
        );
    });
}

#[test]
fn a_server_with_the_right_contract_passes_the_check() {
    with_server("0.1", |socket| {
        let out = check(socket);
        assert_eq!(
            out.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
    });
}

#[test]
fn no_server_ends_the_check_with_1_and_a_message() {
    let dir = tempfile::tempdir().unwrap();
    let out = check(&dir.path().join("nobody.sock"));
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("no fitting server"));
}
