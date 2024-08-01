//! The `vsomeip-rs` crate provides a Rust interface to the [vsomeip C++ library](https://github.com/covESA/vsomeip).
//! Wrapper types are provided for the most important classes and functions.
//! Currently only simple server client communication is supported.
//! See the `simple_client_server` example for a demonstration of how to use the library.

mod application;
mod runtime;
mod message;
mod payload;
mod util;
mod primitives;
mod error;
mod constants;

pub use application::*;
pub use runtime::*;
pub use message::*;
pub use payload::*;
pub use primitives::*;
pub use error::*;
pub use constants::*;