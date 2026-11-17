//! Ending a server through the protocol (`Control.stop`), for a starter that replaces it and for tests.
//!
//! Pattern: Facade over `fs_rpc::stop_server`, with its own runtime so no caller needs one.

use std::path::Path;

use crate::error::Error;

/// Ask the server on `socket` to end in order; answers when it has taken the request (or is gone already).
///
/// # Errors
/// [`Error::Unreachable`] when nothing listens there, the server runs as another user, or it refused.
pub fn stop(socket: &Path) -> Result<(), Error> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| Error::Unreachable(e.to_string()))?;
    let local = tokio::task::LocalSet::new();
    local
        .block_on(&runtime, fs_rpc::stop_server(socket))
        .map_err(|e| Error::Unreachable(e.to_string()))
}
