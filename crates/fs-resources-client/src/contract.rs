//! The contract version a frontend needs, and the check against what the server reports.
//!
//! Pattern: Value object plus a pure function (the rule of ADR 0094 draft, item 4: same main level, minor level of the
//! server not lower; a draft before 1.0 follows the same rule until the contract says otherwise).

use crate::error::Error;

/// What this client needs of the server's contract (`resources.capnp`, draft 0.1).
pub const EXPECTED_CONTRACT: &str = "0.1";

/// The numbers of a contract text: `"0.1"` is `(0, 1)`; a text like `"resources 0.1 (draft)"` is read the same. The first `MAJOR.MINOR` counts.
fn levels(text: &str) -> Option<(u32, u32)> {
    text.split_whitespace().find_map(|word| {
        let (major, minor) = word.split_once('.')?;
        Some((major.parse().ok()?, minor.parse().ok()?))
    })
}

/// Whether a server that reports `found` fits a client that needs `expected`.
///
/// # Errors
/// [`Error::Contract`] when the main level differs, the minor level is lower, or a text holds no version at all.
pub fn check_contract(found: &str, expected: &str) -> Result<(), Error> {
    let bad = || Error::Contract {
        found: found.to_owned(),
        expected: expected.to_owned(),
    };
    let (found_major, found_minor) = levels(found).ok_or_else(bad)?;
    let (need_major, need_minor) = levels(expected).ok_or_else(bad)?;
    if found_major == need_major && found_minor >= need_minor {
        Ok(())
    } else {
        Err(bad())
    }
}

#[cfg(test)]
mod tests {
    use super::{EXPECTED_CONTRACT, check_contract};

    #[test]
    fn the_same_or_a_newer_minor_level_fits() {
        assert!(check_contract("0.1", EXPECTED_CONTRACT).is_ok());
        assert!(check_contract("0.4", EXPECTED_CONTRACT).is_ok());
        assert!(check_contract("resources 0.1 (draft)", EXPECTED_CONTRACT).is_ok());
    }

    #[test]
    fn another_main_level_an_older_minor_level_or_no_number_is_refused() {
        for found in [
            "1.1",
            "0.0",
            "resources 0.0 (draft)",
            "resources",
            "",
            "0.x",
        ] {
            assert!(check_contract(found, EXPECTED_CONTRACT).is_err(), "{found}");
        }
    }
}
