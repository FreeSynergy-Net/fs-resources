//! The names the server and its clients agree on: the program name and the socket.
//!
//! Pattern: none: constants and one function.

use std::path::PathBuf;

/// The program name of the server, as `Control.version` says it.
pub const PROGRAM: &str = "fs-resources";

/// The name the socket is made from (`$XDG_RUNTIME_DIR/freesynergy/resources.sock`).
const SOCKET_PROGRAM: &str = "resources";

/// Where the server listens when nobody says otherwise, the same place `fs-resources serve` binds.
#[must_use]
pub fn default_socket() -> Option<PathBuf> {
    fs_rpc::default_socket(SOCKET_PROGRAM)
}
