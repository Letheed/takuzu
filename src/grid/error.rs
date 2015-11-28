/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::error::Error;
use std::fmt::Display;
use super::Array;

/// An error returned when checking if the grid is well-sized and legal.
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum GridError {
    /// The grid does not have the right size.
    BadSize(GridSizeError),
    /// The grid is illegal, that is it infringes at least one of the rules.
    Illegal,
}

impl Display for GridError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(self.description())
    }
}

impl Error for GridError {
    fn description(&self) -> &str {
        match *self {
            GridError::BadSize(_) => "faulty grid size",
            GridError::Illegal => "grid is illegal",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            GridError::BadSize(ref err) => Some(err),
            GridError::Illegal => None,
        }
    }
}

/// An error returned when parsing a string to create a grid failed.
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum GridParseError {
    /// A `Grid` cannot be created from this `Array`.
    CreationError(GridError, Array),
    /// At least one character other than `0`, `1`, `.` or '\n'
    /// was found in the string.
    UnexpectedCharacter,
}

impl Display for GridParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(self.description())
    }
}

impl Error for GridParseError {
    fn description(&self) -> &str {
        match *self {
            GridParseError::CreationError(_, _) => "could not create grid",
            GridParseError::UnexpectedCharacter => "found unexpected character(s)",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            GridParseError::CreationError(ref err, _) => Some(err),
            GridParseError::UnexpectedCharacter => None,
        }
    }
}

/// An error returned when the grid is not properly sized.
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum GridSizeError {
    /// The grid is empty.
    Empty,
    /// The grid is not a square.
    NotASquare,
    /// The grid has an odd number of rows.
    OddRowNumber,
}

impl Display for GridSizeError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(self.description())
    }
}

impl Error for GridSizeError {
    fn description(&self) -> &str {
        match *self {
            GridSizeError::Empty => "empty grid",
            GridSizeError::NotASquare => "not a square",
            GridSizeError::OddRowNumber => "odd number of rows",
        }
    }
}