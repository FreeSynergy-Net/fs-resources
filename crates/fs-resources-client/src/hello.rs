//! The first conversation with the server: connect, ask what it offers, check the contract.
//!
//! Pattern: Template method: handshake (`Control.version`, replace a server that differs), call, check.

use std::path::Path;

use fs_interfaces::resources_capnp::resources;
use fs_rpc::{Expected, ServerStarter, connect_matching};

use crate::contract::{EXPECTED_CONTRACT, check_contract};
use crate::error::Error;
use crate::names::PROGRAM;

/// What the server said about itself in the first call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hello {
    /// The contract version the server reports (`resources 0.1 (draft)`).
    pub contract: String,
    /// The capability ids the server offers; empty today (everything stays locked in the frontend).
    pub ids: Vec<String>,
}

impl Hello {
    /// Connect to the server on `socket` (the `starter` starts or replaces it when needed), ask `capabilities` and check
    /// the contract version against `expected` (usually [`EXPECTED_CONTRACT`]). Runs its own runtime, so no caller needs one.
    ///
    /// # Errors
    /// [`Error::Unreachable`] when no fitting server could be reached, [`Error::Contract`] when its contract does not fit,
    /// [`Error::Protocol`] when an answer cannot be read.
    pub fn fetch(
        socket: &Path,
        starter: &dyn ServerStarter,
        expected: &str,
    ) -> Result<Self, Error> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| Error::Unreachable(e.to_string()))?;
        let local = tokio::task::LocalSet::new();
        local.block_on(&runtime, async {
            let wanted = Expected::new(env!("CARGO_PKG_VERSION")).with_program(PROGRAM);
            let link = connect_matching::<resources::Client>(socket, &wanted, starter)
                .await
                .map_err(|e| Error::Unreachable(e.to_string()))?;
            let reply = link
                .client
                .capabilities_request()
                .send()
                .promise
                .await
                .map_err(|e| Error::Protocol(e.to_string()))?;
            let reader = reply.get().map_err(|e| Error::Protocol(e.to_string()))?;
            let text = |r: capnp::Result<capnp::text::Reader<'_>>| -> Result<String, Error> {
                r.map_err(|e| Error::Protocol(e.to_string()))?
                    .to_string()
                    .map_err(|e| Error::Protocol(e.to_string()))
            };
            let list = reader
                .get_capabilities()
                .map_err(|e| Error::Protocol(e.to_string()))?;
            let contract = text(list.get_contract_version())?;
            let mut ids = Vec::new();
            for id in list.get_ids().map_err(|e| Error::Protocol(e.to_string()))? {
                ids.push(text(id)?);
            }
            check_contract(&contract, expected)?;
            Ok(Self { contract, ids })
        })
    }

    /// [`Hello::fetch`] against the contract this client was built for.
    ///
    /// # Errors
    /// As for [`Hello::fetch`].
    pub fn fetch_expected(socket: &Path, starter: &dyn ServerStarter) -> Result<Self, Error> {
        Self::fetch(socket, starter, EXPECTED_CONTRACT)
    }
}
