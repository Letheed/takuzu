#![allow(clippy::module_name_repetitions)]

use std::{
    convert::From,
    error::Error,
    fmt::{self, Display},
};

/// An error returned when parsing a string to create a grid failed.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum GridParseError {
    /// The grid does not have the right size.
    /// It should be square, of non-null, even size.
    BadSize(GridSizeError),
    /// At least one character other than `0`, `1`, `.` or `\n`
    /// was found in the string.
    UnexpectedCharacter(char),
}

impl Error for GridParseError {}

impl Display for GridParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GridParseError::BadSize(e) => write!(f, "bad grid size: {}", e),
            GridParseError::UnexpectedCharacter(c) => {
                write!(f, "found unexpected character `{}`", c)
            }
        }
    }
}

impl From<GridSizeError> for GridParseError {
    fn from(err: GridSizeError) -> Self {
        Self::BadSize(err)
    }
}

/// An error returned when checking if the grid is legal.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum GridError {
    /// The grid is illegal, meaning it infringes at least one of the rules.
    Illegal,
}

impl Error for GridError {}

impl Display for GridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GridError::Illegal => write!(f, "grid is illegal"),
        }
    }
}

/// An error returned when the grid is not properly sized.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum GridSizeError {
    /// The grid is empty.
    EmptyGrid,
    /// The grid is not a square.
    NotASquare {
        /// Line on which the error occured.
        line: usize,
        /// Number of characters found.
        found: usize,
        /// Number of characters expected.
        expected: usize,
    },
    /// The size of the grid is an odd number.
    /// The field contains the number of lines in the grid.
    OddNumberSize(usize),
}

impl Error for GridSizeError {}

impl Display for GridSizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GridSizeError::EmptyGrid => write!(f, "grid is empty"),
            GridSizeError::NotASquare { line, found, expected } => {
                write!(
                    f,
                    "grid is not a square (line {}, expected {} characters, found {})",
                    line, expected, found
                )
            }
            GridSizeError::OddNumberSize(n) => {
                write!(f, "grid size is an odd number ({} lines found)", n)
            }
        }
    }
}
