//! `fs-resources-simple`: the standard frontend `simple` of FreeSynergy Resources, as a process of its own.
//!
//! It depends only on the client crate and on fs-components (ADR 0094 draft, item 6): it knows the server through the
//! contract, never through the engine. Today it shows the name, the icon, what the program can do now and what
//! comes; every element that does not work yet is locked with "coming soon" (ADR 0066), and a server that is missing or
//! does not fit is said plainly in the window (ADR 0065), never a grey window.
//!
//! Pattern: Facade: this file only declares modules and re-exports the public API.

#![deny(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod cli;
mod content;
mod link;
mod run;
mod window;
mod words;

pub use cli::{Command, HELP, parse};
pub use content::{blocks, coming, inline, start};
pub use link::Link;
pub use run::{EXIT_CONTRACT, run};
pub use words::{TEXTS, words};
