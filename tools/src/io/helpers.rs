use std::{
    fmt::Display,
    io::{BufRead as _, BufReader},
    process::ChildStdout,
    str::FromStr,
};
use thiserror::Error;

/// Read a token and parse it into a value.
#[allow(dead_code)]
pub(super) fn read<T: Copy + PartialOrd + Display + FromStr>(
    token: Option<&str>,
    lb: T,
    ub: T,
) -> Result<T, TokenParseError<T>> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if v < lb || ub < v {
                Err(TokenParseError::OutOfRange(v))
            } else {
                Ok(v)
            }
        } else {
            Err(TokenParseError::ParseError(v.to_string()))
        }
    } else {
        Err(TokenParseError::UnexpectedEOF)
    }
}

pub(super) fn read_line(
    stdout: &mut BufReader<ChildStdout>,
) -> Result<String, ChildProcessIOError> {
    loop {
        let mut out = String::new();
        let bytes = stdout.read_line(&mut out);

        if bytes.unwrap_or(0) == 0 {
            return Err(ChildProcessIOError::UnexpectedEOF);
        }

        print!("{}", out);

        let v = out.trim();

        if v.len() == 0 || v.starts_with("#") {
            continue;
        }

        return Ok(v.to_owned());
    }
}

#[derive(Debug, Error)]
pub enum TokenParseError<T: Display> {
    #[error("Out of range: {0}")]
    OutOfRange(T),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Unexpected EOF")]
    UnexpectedEOF,
}

#[derive(Debug, Error)]
pub enum ChildProcessIOError {
    #[error("Your program has terminated unexpectedly")]
    UnexpectedEOF,
}
