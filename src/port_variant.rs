// Copyright © 2026 Stephan Kunz
//! Port variants.

#![allow(unused)]

use core::any::Any;

use alloc::boxed::Box;

use crate::{
	ConstString,
	bind::{
		BindCommons, BindIn, BindInOut, BindOut, any_port_value::AnyPortValueType, in_out_port::BoundInOutPort,
		in_port::BoundInPort, out_port::BoundOutPort,
	},
	error::{Error, Result},
};

/// Implemented set of port variants.
/// - InBound: bound to some other ports value, only readable
/// - InOutBound: bound to some other ports value, read- & writeable
/// - OutBound: bound to some other ports value, only writeable
#[allow(clippy::enum_variant_names)] // the flow variants to be expected
#[derive(Debug, Clone)]
pub enum PortVariant {
	InBound(BoundInPort),
	InOutBound(BoundInOutPort),
	OutBound(BoundOutPort),
}

impl PortVariant {
	pub fn create_inbound<T: AnyPortValueType>(value: T) -> Self {
		Self::InBound(BoundInPort::with_value(value))
	}

	pub fn create_inoutbound<T: AnyPortValueType>(value: T) -> Self {
		Self::InOutBound(BoundInOutPort::with_value(value))
	}

	pub fn create_outbound<T: AnyPortValueType>(value: T) -> Self {
		Self::OutBound(BoundOutPort::with_value(value))
	}

	pub fn connect_to(&mut self, other: &PortVariant) -> Result<()> {
		match self {
			Self::InBound(port) => port.bind_to(other),
			Self::InOutBound(port) => port.bind_to(other),
			Self::OutBound(port) => port.bind_to(other),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&PortVariant>();
		is_normal::<PortVariant>();
	}
}
