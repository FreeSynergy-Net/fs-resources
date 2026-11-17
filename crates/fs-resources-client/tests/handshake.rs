//! The client against the real server, in this process: the right contract is accepted, another one is refused.
//!
//! Tests: example-based, on a real socket in a temporary folder of the test target.

use std::path::Path;
use std::thread::JoinHandle;
use std::time::Duration;

use fs_resources_client::{EXPECTED_CONTRACT, Error, Hello};

// `Control.stop` ends every server of this process (fs-rpc keeps one stop flag), so tests that start a server take this lock.
static ONE_SERVER_AT_A_TIME: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn nobody_starts_a_server() -> std::io::Result<()> {
    Err(std::io::Error::other("this test starts its server itself"))
}

fn start(socket: &Path, contract: &'static str) -> JoinHandle<()> {
    let socket = socket.to_owned();
    std::thread::spawn(move || {
        fs_resources_server::run_on(&socket, contract).unwrap();
    })
}

fn wait_for(socket: &Path) {
    for _ in 0..200 {
        if socket.exists() {
            return;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    panic!("the server did not bind {}", socket.display());
}

fn stop(socket: &Path, server: JoinHandle<()>) {
    fs_resources_client::stop(socket).unwrap();
    server.join().unwrap();
}

#[test]
fn a_server_with_the_expected_contract_is_accepted_and_says_it_offers_nothing_yet() {
    let _one = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let dir = tempfile::tempdir().unwrap();
    let socket = dir.path().join("resources.sock");
    let server = start(&socket, fs_resources_server::CONTRACT_VERSION);
    wait_for(&socket);
    let hello = Hello::fetch(&socket, &nobody_starts_a_server, EXPECTED_CONTRACT).unwrap();
    assert_eq!(hello.contract, "0.1");
    assert_eq!(hello.ids, Vec::<String>::new());
    stop(&socket, server);
}

#[test]
fn a_server_with_another_main_level_is_refused_with_both_versions_named() {
    let _one = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let dir = tempfile::tempdir().unwrap();
    let socket = dir.path().join("resources.sock");
    let server = start(&socket, "9.0");
    wait_for(&socket);
    let err = Hello::fetch(&socket, &nobody_starts_a_server, EXPECTED_CONTRACT).unwrap_err();
    match &err {
        Error::Contract { found, expected } => {
            assert_eq!(found, "9.0");
            assert_eq!(expected, EXPECTED_CONTRACT);
        }
        other => panic!("expected a contract error, got {other}"),
    }
    assert!(err.to_string().contains("\"9.0\""));
    stop(&socket, server);
}

#[test]
fn the_server_and_the_client_name_the_same_contract() {
    assert_eq!(fs_resources_server::CONTRACT_VERSION, EXPECTED_CONTRACT);
}

#[test]
fn no_server_and_a_starter_that_cannot_start_one_is_a_visible_error() {
    let dir = tempfile::tempdir().unwrap();
    let socket = dir.path().join("none.sock");
    let err = Hello::fetch(&socket, &nobody_starts_a_server, EXPECTED_CONTRACT).unwrap_err();
    assert!(matches!(err, Error::Unreachable(_)), "{err}");
}
