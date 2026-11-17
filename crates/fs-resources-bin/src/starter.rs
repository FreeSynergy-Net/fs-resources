//! The starter: make sure the server runs, pick the frontend, open it; fall back to the standard frontend, never silently.
//!
//! Pattern: Strategy (`decide` is a pure function over the frontend's answer) + Facade over the client crate (ADR 0094 draft, item 4).

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

use fs_resources_client::{CommandStarter, Hello, default_socket};

/// The frontend that always comes with the program (ADR 0074 §8).
pub const STANDARD: &str = "simple";

/// The exit code a frontend uses for "the server's contract does not fit" (ADR 0094 draft, item 4).
pub const EXIT_CONTRACT: i32 = 3;

/// What the starter does after asking a frontend to check the server.
#[derive(Debug, PartialEq, Eq)]
pub enum Next {
    /// Open this frontend.
    Open(String),
    /// Say `message` to the person, then open the standard frontend.
    Fallback {
        /// What the person is told.
        message: String,
    },
}

/// `check` is the exit code of `<frontend> --check`, or `None` when the frontend could not be run at all.
#[must_use]
pub fn decide(chosen: &str, check: Option<i32>) -> Next {
    match check {
        None if chosen != STANDARD => Next::Fallback {
            message: format!("the frontend {chosen} cannot be started; opening {STANDARD} instead"),
        },
        Some(EXIT_CONTRACT) if chosen != STANDARD => Next::Fallback {
            message: format!(
                "the frontend {chosen} does not fit the contract of this server; opening {STANDARD} instead"
            ),
        },
        // The standard frontend shows a server that is missing or does not fit in its own window.
        _ => Next::Open(chosen.to_owned()),
    }
}

struct Words {
    frontend: String,
    socket: Option<PathBuf>,
}

fn read(words: &[OsString]) -> Result<Words, String> {
    let (mut frontend, mut socket) = (STANDARD.to_owned(), None);
    let mut it = words.iter();
    while let Some(word) = it.next() {
        let word = word.to_string_lossy().into_owned();
        let mut value = || {
            it.next()
                .map(|v| v.to_string_lossy().into_owned())
                .filter(|v| !v.is_empty() && !v.starts_with("--"))
                .ok_or_else(|| format!("{word} needs a value"))
        };
        match word.as_str() {
            "--frontend" => {
                let id = value()?;
                if !id
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
                {
                    return Err(format!("{id} is no frontend id (lower-case ASCII)"));
                }
                frontend = id;
            }
            "--socket" => socket = Some(PathBuf::from(value()?)),
            other => return Err(format!("unknown option {other}")),
        }
    }
    Ok(Words { frontend, socket })
}

/// Where the executable of a frontend lives: next to this program, else in `/usr/bin`; never found through `PATH`.
fn exe_of(frontend: &str) -> PathBuf {
    let name = format!("fs-resources-{frontend}");
    let beside = std::env::current_exe()
        .ok()
        .and_then(|me| me.parent().map(|dir| dir.join(&name)))
        .filter(|path| path.is_file());
    beside.unwrap_or_else(|| PathBuf::from("/usr/bin").join(name))
}

fn code_of(status: std::io::Result<std::process::ExitStatus>) -> Option<i32> {
    status.ok().and_then(|s| s.code())
}

/// The whole start: the server, the check, the frontend.
pub fn run(words: &[OsString]) -> ExitCode {
    let Words { frontend, socket } = match read(words) {
        Ok(w) => w,
        Err(m) => {
            eprintln!("fs-resources: {m}\nTry `fs-resources --help`.");
            return ExitCode::from(2);
        }
    };
    let Some(socket) = socket.or_else(default_socket) else {
        eprintln!("fs-resources: no default socket (no $XDG_RUNTIME_DIR): pass --socket");
        return ExitCode::FAILURE;
    };
    // The server first: start it if it does not run, replace one of another version (the handshake of fs-rpc).
    if let Ok(me) = std::env::current_exe() {
        let server = CommandStarter::new(
            me,
            [
                "serve".into(),
                "--socket".into(),
                socket.clone().into_os_string(),
            ],
        );
        if let Err(problem) = Hello::fetch_expected(&socket, &server) {
            // Not fatal: the standard frontend says the same in its window. The person also gets it here.
            eprintln!("fs-resources: {problem}");
        }
    }
    let check = |id: &str| {
        code_of(
            Command::new(exe_of(id))
                .arg("--check")
                .arg("--socket")
                .arg(&socket)
                .status(),
        )
    };
    let open = match decide(&frontend, check(&frontend)) {
        Next::Open(id) => id,
        Next::Fallback { message } => {
            eprintln!("fs-resources: {message}");
            STANDARD.to_owned()
        }
    };
    match Command::new(exe_of(&open))
        .arg("--socket")
        .arg(&socket)
        .status()
    {
        Ok(status) => ExitCode::from(u8::try_from(status.code().unwrap_or(1)).unwrap_or(1)),
        Err(e) => {
            eprintln!("fs-resources: cannot start the frontend {open}: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EXIT_CONTRACT, Next, STANDARD, decide};

    #[test]
    fn a_frontend_that_fits_is_opened() {
        assert_eq!(decide("other", Some(0)), Next::Open("other".into()));
    }

    #[test]
    fn a_frontend_with_a_wrong_contract_falls_back_with_a_message() {
        match decide("other", Some(EXIT_CONTRACT)) {
            Next::Fallback { message } => {
                assert!(message.contains("other") && message.contains(STANDARD));
            }
            Next::Open(id) => panic!("opened {id}"),
        }
    }

    #[test]
    fn a_frontend_that_cannot_be_run_falls_back_too() {
        assert!(matches!(decide("other", None), Next::Fallback { .. }));
    }

    #[test]
    fn the_standard_frontend_is_opened_anyway_because_its_window_says_what_is_wrong() {
        assert_eq!(
            decide(STANDARD, Some(EXIT_CONTRACT)),
            Next::Open(STANDARD.into())
        );
        assert_eq!(decide(STANDARD, Some(1)), Next::Open(STANDARD.into()));
    }
}
