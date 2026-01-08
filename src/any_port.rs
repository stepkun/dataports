// Copyright © 2026 Stephan Kunz
//! An abstract port type trait.

#![allow(unused)]

use core::any::Any;

use alloc::boxed::Box;

use crate::{
	ConstString, PortCommons,
	bind::{in_out_port::BoundInOutPort, in_port::BoundInPort, out_port::BoundOutPort},
	flow::{in_out_port::FlowingInOutPort, in_port::FlowingInPort, out_port::FlowingOutPort},
};

/// The `AnyPortType` trait allows to use different types of ports in collections.
pub trait AnyPortType: Any + Send + Sync + core::fmt::Debug {
	/// Convert to Any
	#[must_use]
	fn as_any(&self) -> &dyn Any;

	/// Convert to mut Any
	#[must_use]
	fn as_mut_any(&mut self) -> &mut dyn Any;
}

macro_rules! implement_any_port_type {
	($($tp:ty),*) => {
		$(impl AnyPortType for $tp {
			fn as_any(&self) -> &dyn Any {
				self
			}
			fn as_mut_any(&mut self) -> &mut dyn Any {
				self
			}
		})*
	};
}

implement_any_port_type!(BoundInOutPort, BoundInPort, BoundOutPort);
implement_any_port_type!(FlowingInOutPort, FlowingInPort, FlowingOutPort);
