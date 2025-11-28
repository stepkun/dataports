// Copyright © 2025 Stephan Kunz
#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

extern crate alloc;

mod error;
mod macros;
mod port;
mod port_provider;
mod remapping;

// flatten
pub use error::Error;
pub use port::{Port, PortBase, InPort, InOutPort, OutPort};
pub use port_provider::{PortProvider, DynamicPortProvider};