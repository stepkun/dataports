// Copyright © 2025 Stephan Kunz

#![no_std]
#![doc = include_str!("../README.md")]
#![allow(unused)]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

extern crate alloc;

use alloc::sync::Arc;

// internal re-export for easy changeability
use spin::{RwLock, RwLockReadGuard, RwLockWriteGuard};

// flatten
pub use bind::in_out_port::BoundInOutPort;
pub use bind::in_port::BoundInPort;
pub use bind::out_port::BoundOutPort;
pub use collections::port_array::PortArray;
pub use flow::in_out_port::FlowingInOutPort;
pub use flow::in_port::FlowingInPort;
pub use flow::out_port::FlowingOutPort;
pub use port::Port;

use crate::bind::BindIn;

// internal module structure
mod any_port;
mod bind;
mod collections;
mod error;
mod flow;
mod port;

/// An immutable thread safe `String` type
/// see: [Logan Smith](https://www.youtube.com/watch?v=A4cKi7PTJSs).
type ConstString = Arc<str>;

trait PortCommons {
	fn name(&self) -> ConstString;
}
