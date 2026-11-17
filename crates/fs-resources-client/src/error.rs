//! Why a frontend could not use the server, in words for a person (ADR 0065: a visible message, never a black window).
//!
//! Pattern: none: a plain error enum.

use std::fmt;

/// Why no usable server was reached.
#[derive(Debug)]
pub enum Error {
    /// The server speaks another contract version than this client needs; `found` is what it said.
    Contract {
        /// What the server reported.
        found: String,
        /// What this client needs at least.
        expected: String,
    },
    /// No fitting server could be reached (not running, another user, another program on the socket, a restart failed).
    Unreachable(String),
    /// The server answered something this client cannot read.
    Protocol(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Contract { found, expected } => write!(
                f,
                "the server speaks \"{found}\", this frontend needs \"{expected}\" (same main level, minor level not lower)"
            ),
            Self::Unreachable(why) => write!(f, "no fitting server: {why}"),
            Self::Protocol(why) => write!(f, "the server answered something unreadable: {why}"),
        }
    }
}

impl std::error::Error for Error {}
