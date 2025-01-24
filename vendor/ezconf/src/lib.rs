//! ezconf
//! ======
//!
//! A library to add configuration options to your project with as little
//! boilerplate as possible. Uses `toml` as the configuration format.
//!
//! *Note*: In previous versions, values were cached.  This is no longer the
//! case!  If you need maximum performance, call `get` before doing anything
//! time-critical.
//!
//! # Example
//! ```
//! extern crate ezconf;
//!
//! static CONFIG: ezconf::Config = ezconf::INIT;
//!
//! fn main() {
//!     CONFIG
//!         .init([ezconf::Source::File("tests/test.toml")].iter())
//!         .unwrap();
//!
//!     let v = CONFIG.get_or::<String>("string.a", "Hello String".into());
//!     println!("Value: {:?}", v);
//! }
//! ```
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
)]

#[macro_use]
extern crate log;
extern crate once_cell;
pub extern crate toml;
extern crate toml_query;

pub mod config;
pub use config::{Config, INIT};

pub mod source;
pub use source::Source;
