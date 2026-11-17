//! `fs-resources serve`: the options and the serving, as functions, so one binary can hold server and starter.
//!
//! Pattern: Facade: reads the options, then serves until a signal or `Control.stop`.

use std::ffi::OsString;
use std::io;
use std::path::Path;

use fs_rpc::{ServerIdentity, ServerOptions, run_until_signal_as};

use crate::root::{CONTRACT_VERSION, PROGRAM, Root};

/// The help text of `serve`.
pub const HELP: &str = "fs-resources serve [--socket PATH] [--data DIR] [--actor IRI] [--instance DOMAIN] [--facet NAME]\nDefaults: $XDG_RUNTIME_DIR/freesynergy/resources.sock, $XDG_DATA_HOME/freesynergy/resources, the local owner\nServes the contract resources.capnp (draft 0.1); stops on SIGTERM, Ctrl-C or Control.stop.";

/// The name the socket is made from: `$XDG_RUNTIME_DIR/freesynergy/resources.sock` (the client says the same).
const SOCKET_PROGRAM: &str = "resources";

/// Why `serve` ended without serving.
#[derive(Debug, PartialEq, Eq)]
pub enum ServeError {
    /// The words were wrong; the caller exits with 2.
    Usage(String),
    /// The server could not start or stopped with an error; exit with 1.
    Failed(String),
}

/// Serve the contract until a signal arrives. `args` are the words after `serve`.
///
/// # Errors
/// [`ServeError::Usage`] for unknown or incomplete words, [`ServeError::Failed`] for everything else.
pub fn serve(args: impl IntoIterator<Item = OsString>) -> Result<(), ServeError> {
    let Some(options) = ServerOptions::parse(SOCKET_PROGRAM, args).map_err(ServeError::Usage)?
    else {
        println!("{HELP}");
        return Ok(());
    };
    run_on(&options.socket, CONTRACT_VERSION).map_err(|e| ServeError::Failed(e.to_string()))
}

/// Serve on `socket`, reporting `contract` as the contract version, until a signal or `Control.stop`. Runs its own runtime.
///
/// # Errors
/// The socket could not be bound, or work under way did not finish in time.
pub fn run_on(socket: &Path, contract: &str) -> io::Result<()> {
    // Name and version go to `Control.version`: a frontend of another version replaces this server (fs-rpc handshake).
    let identity = ServerIdentity::new(PROGRAM, env!("CARGO_PKG_VERSION"));
    let contract = contract.to_owned();
    run_until_signal_as(socket, identity, move || async move {
        Ok(Root::new(contract).into_client())
    })
}
