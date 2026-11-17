//! The command line of `fs-resources-simple`: a process the starter runs; a person never types it.
//!
//! Pattern: Command: the words become one `Command`, or a message for exit code 2.

use std::path::PathBuf;

/// The help text.
pub const HELP: &str = "fs-resources-simple [--socket PATH] [--check] [--picture FILE.png [--size WIDTHxHEIGHT] [--screen start|comes]]\n\
The standard frontend `simple` of FreeSynergy Resources. Start the program with `fs-resources`, not this one.\n\
  --socket PATH    the server's socket (default: $XDG_RUNTIME_DIR/freesynergy/resources.sock)\n\
  --check          only connect and check the contract version, then exit (0 fits, 3 contract differs, 1 no server)\n\
  --picture FILE   draw the window to a picture, without opening it\n\
  --screen ID      the page to draw (start or comes; default start)\n\
  --version, -V    print the version and exit\n\
  --help, -h       print this help and exit\n\
Exit codes: 0 done, 1 failed, 2 wrong words, 3 the server's contract does not fit";

/// What to do.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    /// Print the version.
    Version,
    /// Print the help.
    Help,
    /// Connect, check the contract, exit.
    Check {
        /// The server's socket, if named.
        socket: Option<PathBuf>,
    },
    /// Open the window.
    Window {
        /// The server's socket, if named.
        socket: Option<PathBuf>,
        /// Draw to this picture instead of opening a window.
        picture: Option<(PathBuf, (u32, u32))>,
        /// The screen to show first: `start` or `comes`.
        screen: Option<String>,
    },
}

/// Read the words after the program name.
///
/// # Errors
/// A message for the person: an unknown option, a missing or wrong value.
pub fn parse(words: &[String]) -> Result<Command, String> {
    let (mut socket, mut check, mut picture, mut size) = (None, false, None, (1000, 760));
    let mut screen = None;
    let mut it = words.iter();
    while let Some(word) = it.next() {
        let mut value = |name: &str| {
            it.next()
                .cloned()
                .ok_or_else(|| format!("{name} needs a value"))
        };
        match word.as_str() {
            "--version" | "-V" => return Ok(Command::Version),
            "--help" | "-h" => return Ok(Command::Help),
            "--check" => check = true,
            "--socket" => socket = Some(PathBuf::from(value("--socket")?)),
            "--picture" => picture = Some(PathBuf::from(value("--picture")?)),
            "--screen" => {
                screen = Some(match value("--screen")?.as_str() {
                    "start" => "start".to_owned(),
                    "comes" => "comes".to_owned(),
                    other => return Err(format!("--screen wants start or comes, not {other}")),
                });
            }
            "--size" => size = parse_size(&value("--size")?)?,
            other => return Err(format!("unknown option {other}")),
        }
    }
    Ok(if check {
        Command::Check { socket }
    } else {
        Command::Window {
            socket,
            picture: picture.map(|p| (p, size)),
            screen,
        }
    })
}

fn parse_size(text: &str) -> Result<(u32, u32), String> {
    let bad = || format!("--size wants WIDTHxHEIGHT, not {text}");
    let (w, h) = text.split_once('x').ok_or_else(bad)?;
    let (w, h): (u32, u32) = (w.parse().map_err(|_| bad())?, h.parse().map_err(|_| bad())?);
    if (1..=8192).contains(&w) && (1..=8192).contains(&h) {
        Ok((w, h))
    } else {
        Err(bad())
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, parse};

    fn words(text: &str) -> Vec<String> {
        text.split_whitespace().map(str::to_owned).collect()
    }

    #[test]
    fn the_known_words_are_read() {
        assert_eq!(parse(&words("--version")), Ok(Command::Version));
        assert_eq!(parse(&words("-h")), Ok(Command::Help));
        assert!(matches!(
            parse(&words("--check --socket /x")),
            Ok(Command::Check { socket: Some(_) })
        ));
        assert!(matches!(
            parse(&words("--picture a.png --size 800x600")),
            Ok(Command::Window {
                picture: Some((_, (800, 600))),
                ..
            })
        ));
        assert!(matches!(
            parse(&[]),
            Ok(Command::Window {
                socket: None,
                picture: None,
                screen: None
            })
        ));
        assert!(matches!(
            parse(&words("--screen comes")),
            Ok(Command::Window { screen: Some(s), .. }) if s == "comes"
        ));
        assert!(parse(&words("--screen nope")).is_err());
    }

    #[test]
    fn unknown_words_and_missing_values_are_errors() {
        assert!(
            parse(&words("--nope"))
                .unwrap_err()
                .contains("unknown option --nope")
        );
        assert!(parse(&words("--socket")).is_err());
        assert!(parse(&words("--size 0x5 --picture p")).is_err());
    }
}
