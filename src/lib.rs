//! This crate allows you to derive a `std::str::FromStr` implementation based on a regex
//! provided via macro attribute.
//!
//! ```edition2018
//! use adhoc_derive::FromStr;
//! #[derive(FromStr)]
//! #[adhoc(regex = r"^#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)$")]
//! struct Rectangle {
//!     id: usize,
//!     x: usize,
//!     y: usize,
//!     width: usize,
//!     height: usize,
//! }
//!
//! let rect: Rectangle = "#123 @ 3,2: 5x4".parse().unwrap();
//! assert_eq!(123, rect.id);
//! assert_eq!(3, rect.x);
//! assert_eq!(2, rect.y);
//! assert_eq!(5, rect.width);
//! assert_eq!(4, rect.height);
//! ```
//!
//! Refer to [GUIDE.md](https://github.com/df5602/adhoc_derive/blob/master/GUIDE.md) for more examples.

#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod adhoc;

#[proc_macro_derive(FromStr, attributes(adhoc))]
pub fn from_str_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    adhoc::from_str_derive(input)
}
