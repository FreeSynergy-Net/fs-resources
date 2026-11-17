//! Running a command: the exit codes of the frontend (ADR 0094 draft, item 4: a wrong contract ends with 3 and a message).
//!
//! Pattern: Facade over the client crate and the window.

use std::path::PathBuf;
use std::process::ExitCode;

use fs_components_app::{WindowOptions, launch};
use fs_resources_client::{Error, default_socket};

use crate::cli::{Command, HELP, parse};
use crate::link::Link;
use crate::window::app;
use crate::words::{session_locale, words};

/// The exit code for "the server's contract does not fit": the starter reads it and starts the standard frontend instead.
pub const EXIT_CONTRACT: u8 = 3;

fn socket_of(named: Option<PathBuf>) -> Result<PathBuf, String> {
    named
        .or_else(default_socket)
        .ok_or_else(|| "no default socket (no $XDG_RUNTIME_DIR): pass --socket".to_owned())
}

/// Run the command line (the words after the program name) and return the exit code.
#[must_use]
pub fn run(args: &[String]) -> ExitCode {
    let command = match parse(args) {
        Ok(command) => command,
        Err(message) => {
            eprintln!("fs-resources-simple: {message}\nTry `fs-resources-simple --help`.");
            return ExitCode::from(2);
        }
    };
    match command {
        Command::Version => {
            println!("fs-resources-simple {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Command::Help => {
            println!("{HELP}");
            ExitCode::SUCCESS
        }
        Command::Check { socket } => check(socket),
        Command::Window {
            socket,
            picture,
            screen,
        } => window(socket, picture, screen.as_deref()),
    }
}

fn check(socket: Option<PathBuf>) -> ExitCode {
    let socket = match socket_of(socket) {
        Ok(s) => s,
        Err(m) => {
            eprintln!("fs-resources-simple: {m}");
            return ExitCode::FAILURE;
        }
    };
    match Link::probe(&socket) {
        Link::Connected { contract } => {
            println!("fs-resources-simple: the server speaks {contract}: fits");
            ExitCode::SUCCESS
        }
        Link::Mismatch(reason) => {
            eprintln!("fs-resources-simple: {reason}");
            ExitCode::from(EXIT_CONTRACT)
        }
        Link::Down(reason) => {
            eprintln!("fs-resources-simple: {reason}");
            ExitCode::FAILURE
        }
    }
}

fn window(
    socket: Option<PathBuf>,
    picture: Option<(PathBuf, (u32, u32))>,
    screen: Option<&str>,
) -> ExitCode {
    // A server that is missing or does not fit does not keep the window closed: the window says so (ADR 0065).
    let link = match socket_of(socket) {
        Ok(socket) => Link::probe(&socket),
        Err(m) => Link::of(Err(Error::Unreachable(m))),
    };
    let widget = match app(&words(&session_locale()), &link) {
        Ok(widget) => widget,
        Err(e) => {
            eprintln!("fs-resources-simple: {e}");
            return ExitCode::FAILURE;
        }
    };
    if let Some(id) = screen {
        widget.select(id);
    }
    let done = match picture {
        Some((path, (w, h))) => widget.picture(w, h, &path),
        None => launch(WindowOptions::default(), widget),
    };
    match done {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("fs-resources-simple: {e}");
            ExitCode::FAILURE
        }
    }
}
