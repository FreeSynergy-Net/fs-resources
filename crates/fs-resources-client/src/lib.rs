//! `fs-resources-client`: the program as a client of its own contract `resources.capnp` (draft 0.1).
//!
//! A frontend depends on this crate and talks to a running `fs-resources serve`, never to the engine behind it (ADR 0021,
//! ADR 0094 draft). [`Hello::fetch`] connects (starting or replacing the server through the program's starter), asks
//! what the server offers and checks the contract version ([`check_contract`]).
//!
//! Interface status: draft (ADR 0045), as the contract.
//!
//! Pattern: Facade: this file only declares modules and re-exports the public API.

#![deny(missing_docs)]
#![warn(clippy::pedantic)]

mod contract;
mod error;
mod hello;
mod names;
mod stop;

pub use contract::{EXPECTED_CONTRACT, check_contract};
pub use error::Error;
pub use fs_rpc::{CommandStarter, ServerStarter};
pub use hello::Hello;
pub use names::{PROGRAM, default_socket};
pub use stop::stop;
