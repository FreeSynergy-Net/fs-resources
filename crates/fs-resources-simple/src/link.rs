//! What the window knows about its server: the one state it shows at the top.
//!
//! Pattern: Value object; the state is decided once, by the handshake of the client crate.

use fs_resources_client::{EXPECTED_CONTRACT, Error, Hello, ServerStarter};
use std::path::Path;

/// The state of the link to the server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Link {
    /// The server answered and speaks a contract this frontend understands.
    Connected {
        /// The contract version the server reported.
        contract: String,
    },
    /// The server speaks another contract version; the person is told, the window shows only what is planned.
    Mismatch(String),
    /// No fitting server could be reached.
    Down(String),
}

impl Link {
    /// Ask the server on `socket` (never starting one: that is the starter's job, ADR 0094 draft item 4).
    #[must_use]
    pub fn probe(socket: &Path) -> Self {
        let nothing: &dyn ServerStarter = &|| {
            Err(std::io::Error::other(
                "this frontend does not start servers",
            ))
        };
        Self::of(Hello::fetch(socket, nothing, EXPECTED_CONTRACT))
    }

    /// The state for one answer of the client.
    #[must_use]
    pub fn of(answer: Result<Hello, Error>) -> Self {
        match answer {
            Ok(hello) => Self::Connected {
                contract: hello.contract,
            },
            Err(error @ Error::Contract { .. }) => Self::Mismatch(error.to_string()),
            Err(other) => Self::Down(other.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Link;
    use fs_resources_client::{Error, Hello};

    #[test]
    fn each_answer_gets_its_own_state() {
        let ok = Hello {
            contract: "0.1".into(),
            ids: vec![],
        };
        assert!(matches!(Link::of(Ok(ok)), Link::Connected { .. }));
        let bad = Error::Contract {
            found: "x".into(),
            expected: "y".into(),
        };
        assert!(matches!(Link::of(Err(bad)), Link::Mismatch(_)));
        assert!(matches!(
            Link::of(Err(Error::Unreachable("no".into()))),
            Link::Down(_)
        ));
    }
}
