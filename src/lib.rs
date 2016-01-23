/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! A Takuzu (a.k.a. Binairo) solving library.
//!
//! If you are looking for a binary, see [tackle](../tackle/index.html)
//! for an example of implementation.
//!
//! # About
//!
//! Takuzu is a number puzzle, sometimes called binary sudoku.
//! The objective is to fill a grid with `0`s and `1`s while
//! observing the following rules:
//!
//! * no more than two of either number adjacent to each other in one direction.
//! * each row and each column should contain an equal number of `0`s and `1`s.
//! * no two rows and no two columns can be the same.
//!
//! The grids are squares of even size.
//! A valid grid must have one and only one solution.
//! The solver will find and return all the valid solutions though.
//!
//! # Format
//!
//! The grid should be represented with the following characters:
//! `0`, `1`, `.` for a missing number and one `\n` at the end of each row.
//! (The final `\n` can be omitted though.)
//!
//! [Example grids](https://github.com/Letheed/takuzu/tree/master/grids)
//!
//! ## Sources
//!
//! Check out the [repository](https://github.com/Letheed/takuzu).


#![warn(missing_docs)]

pub use grid::{Array, Grid};
pub use grid::error::{GridError, GridParseError};
pub use source::Source;
pub use source::error::SourceError;

mod grid;
mod source;
