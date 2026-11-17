//! The frontend depends on the client and fs-components only: no way leads to the core or the server (ADR 0094 draft, item 6).
//!
//! Tests: example-based, reads `cargo tree` of the real manifest (normal dependencies; the server is a dev-dependency for the handshake tests only).

use std::process::Command;

#[test]
fn no_normal_dependency_leads_to_the_core_or_the_server() {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let out = Command::new(cargo)
        .args([
            "tree",
            "-p",
            "fs-resources-simple",
            "-e",
            "normal",
            "--prefix",
            "none",
            "--locked",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let tree = String::from_utf8_lossy(&out.stdout);
    assert!(
        tree.contains("fs-resources-client"),
        "the client must be there: {tree}"
    );
    for forbidden in ["fs-resources-core", "fs-resources-server"] {
        assert!(
            !tree.lines().any(|l| l.starts_with(forbidden)),
            "{forbidden} is a dependency"
        );
    }
}
