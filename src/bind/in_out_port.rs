// Copyright © 2026 Stephan Kunz
//! A bound input type port implementing [`BindIn`].

use alloc::boxed::Box;

use crate::bind::{BindCommons, BindIn, BindOut, any_port_value::AnyPortValueType, port_value::PortValue};

/// @TODO:
#[derive(Debug)]
pub struct BoundInOutPort(Box<dyn AnyPortValueType>);

impl BoundInOutPort {
	pub fn new<T: AnyPortValueType>(value: T) -> Self {
		Self(Box::new(PortValue::<T>::new(value)))
	}
}

impl BindIn for BoundInOutPort {
	fn get<T>(&self) -> Option<T>
	where
		T: Clone,
	{
		todo!()
	}

	fn read<T>(&self) -> crate::error::Result<super::port_value::PortValueReadGuard<T>> {
		todo!()
	}

	fn try_read<T>(&self) -> crate::error::Result<super::port_value::PortValueReadGuard<T>> {
		todo!()
	}
}

impl BindCommons for BoundInOutPort {}

impl BindOut for BoundInOutPort {
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

impl Clone for BoundInOutPort {
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
		is_normal::<&BoundInOutPort>();
		is_normal::<BoundInOutPort>();
	}
}
