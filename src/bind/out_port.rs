// Copyright © 2026 Stephan Kunz
//! A bound output type port implementing [`BindOut`].

use alloc::boxed::Box;

use crate::bind::{BindCommons, BindOut, any_port_value::AnyPortValueType, port_value::PortValue};

/// @TODO:
#[derive(Debug)]
pub struct BoundOutPort(Box<dyn AnyPortValueType>);

impl BoundOutPort {
	pub fn new<T: AnyPortValueType>(value: T) -> Self {
		Self(Box::new(PortValue::<T>::new(value)))
	}
}

impl BindCommons for BoundOutPort {}

impl BindOut for BoundOutPort {
	fn set<T>(&mut self, value: impl Into<T>) {
		todo!()
	}

	fn write<T>(&mut self) -> crate::error::Result<super::port_value::PortValueWriteGuard<T>> {
		todo!()
	}

	fn try_write<T>(&mut self) -> crate::error::Result<super::port_value::PortValueWriteGuard<T>> {
		todo!()
	}
}

impl Clone for BoundOutPort {
	fn clone(&self) -> Self {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&BoundOutPort>();
		is_normal::<BoundOutPort>();
	}
}
