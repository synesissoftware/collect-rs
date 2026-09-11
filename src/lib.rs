//! Specialized collections and containers for Rust.
//!
//! **collect-rs** currently provides frequency maps for generic values and
//! Unicode code points. The public types are exposed through the
//! [`containers`] module; the crate root does not re-export them.
//!
//! # Installation
//!
//! Reference the crate in **Cargo.toml** in the usual way:
//!
//! ```toml
//! collect-rs = { version = "0.2.1" }
//! ```
//!
//! # Components
//!
//! ## Containers
//!
//! * [`containers::FrequencyMap`] — counts occurrences of generic keys;
//! * [`containers::UnicodePointMap`] — counts Unicode code points;
//!
//! The public [`collections`] and [`utils`] modules are currently empty
//! barrels and expose no supported items.
//!
//! # Examples
//!
//! ```rust
//! use collect_rs::containers::FrequencyMap;
//!
//! let mut frequencies = FrequencyMap::default();
//! frequencies.push("cat");
//! frequencies.push("cat");
//!
//! assert_eq!(2, frequencies.get("cat"));
//! ```
//!
//! See the project [README](https://github.com/synesissoftware/collect-rs)
//! for further information.

// lib.rs : collect-rs


// /////////////////////////////////////////////////////////
// crate-level feature definitions


// /////////////////////////////////////////////////////////
// crate-level feature discrimination


// /////////////////////////////////////////////////////////
// imports


pub mod collections;
pub mod containers;
pub(crate) mod macros;
pub mod utils;


// ///////////////////////////// end of file //////////////////////////// //
