//! `fs-resources-server`: the contract `resources.capnp` (draft 0.1, `fs-interfaces` v0.0.27) over a local socket.
//!
//! Today the server answers two methods of the root capability: `capabilities` (what it offers: nothing yet, and the
//! contract version) and `supportReport`. Every other method answers `unimplemented`; a frontend shows what the server
//! does not offer as locked ("kommt noch", ADR 0066). The core (things, reservations, capacity, needs and offers) stays in its crate until the
//! contract is wired to them (`DESIGN.md`).
//!
//! Interface status: draft (ADR 0045), as the contract.
//!
//! Pattern: Facade: this file only declares modules and re-exports the public API.

#![deny(missing_docs)]
#![warn(clippy::pedantic)]
// The generated `Server` trait is async for every method, also those that never wait.
#![allow(clippy::unused_async_trait_impl)]

mod program;
mod root;

pub use program::{HELP, ServeError, run_on, serve};
pub use root::{CONTRACT_VERSION, PROGRAM, Root};
