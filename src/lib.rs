// Copyright © 2025 Stephan Kunz

#![no_std]
#![doc = include_str!("../README.md")]
#![allow(unused)]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

extern crate alloc;

use core::any::Any;

use alloc::sync::Arc;

// internal re-export for easy changeability
use spin::{RwLock, RwLockReadGuard, RwLockWriteGuard};

// flatten
pub use bind::{
	BindCommons, BindIn, BindInOut, BindOut,
	in_out_port::BoundInOutPort,
	in_port::BoundInPort,
	out_port::BoundOutPort,
	port_value::{PortValueReadGuard, PortValueWriteGuard},
};
pub use collections::{PortProvider, port_array::PortArray, port_list::PortList, port_map::PortMap};
//pub use flow::{in_out_port::FlowingInOutPort, in_port::FlowingInPort, out_port::FlowingOutPort};
pub use port_variant::PortVariant;

use crate::error::Result;

// internal module structure
mod bind;
mod collections;
mod error;
//mod flow;
mod port_variant;

/// An immutable thread safe `String` type
/// see: [Logan Smith](https://www.youtube.com/watch?v=A4cKi7PTJSs).
type ConstString = Arc<str>;
