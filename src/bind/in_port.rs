// Copyright © 2026 Stephan Kunz
//! A bound input type port implementing [`BindIn`].

use alloc::boxed::Box;

use crate::bind::{BindCommons, BindIn, any_port_value::AnyPortValueType, port_value::PortValue};

/// @TODO:
#[derive(Debug)]
pub struct BoundInPort(Box<dyn AnyPortValueType>);

impl BoundInPort {
	pub fn new<T: AnyPortValueType>(value: T) -> Self {
		Self(Box::new(PortValue::<T>::new(value)))
	}
}

impl BindCommons for BoundInPort {}

impl BindIn for BoundInPort {
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

impl Clone for BoundInPort {
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
		is_normal::<&BoundInPort>();
		is_normal::<BoundInPort>();
	}
}
