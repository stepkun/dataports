// Copyright © 2026 Stephan Kunz
//! A bound input type port implementing [`BindIn`].

use alloc::{boxed::Box, sync::Arc};
use spin::RwLock;

use crate::{
	bind::{
		BindCommons, BindIn, BindInOut, BindOut,
		any_port_value::AnyPortValueType,
		port_value::{PortValue, PortValuePtr, PortValueReadGuard, PortValueWriteGuard},
	},
	error::Result,
};

/// @TODO:
#[derive(Debug)]
pub struct BoundInOutPort(PortValuePtr);

impl BindCommons for BoundInOutPort {}

impl BoundInOutPort {
	pub fn new<T: AnyPortValueType>(value: T) -> Self {
		Self(Arc::new(RwLock::new(PortValue::<T>::new(value))))
	}
}

impl<T: AnyPortValueType> BindIn<T> for BoundInOutPort {
	fn get(&self) -> Option<T>
	where
		T: Clone,
	{
		let any_value = &*self.0.read();
		if let Some(t_ref) = any_value.as_any().downcast_ref::<PortValue<T>>() {
			t_ref.get()
		} else {
			None
		}
	}

	fn read(&self) -> crate::error::Result<PortValueReadGuard<T>> {
		PortValueReadGuard::new(self.0.clone())
	}

	fn try_read(&self) -> crate::error::Result<PortValueReadGuard<T>> {
		PortValueReadGuard::try_new(self.0.clone())
	}
}

impl<T: AnyPortValueType> BindInOut<T> for BoundInOutPort {
	fn replace(&mut self, value: T) -> Option<T> {
		let mut any_value = &mut *self.0.write();
		if let Some(t_ref) = any_value
			.as_mut_any()
			.downcast_mut::<PortValue<T>>()
		{
			t_ref.replace(value)
		} else {
			None
		}
	}

	fn take(&mut self) -> Option<T> {
		let mut any_value = &mut *self.0.write();
		if let Some(t_ref) = any_value
			.as_mut_any()
			.downcast_mut::<PortValue<T>>()
		{
			t_ref.take()
		} else {
			None
		}
	}
}

impl<T: AnyPortValueType> BindOut<T> for BoundInOutPort {
	fn set(&mut self, value: T) -> Result<()> {
		let mut any_value = &mut *self.0.write();
		if let Some(t_ref) = any_value
			.as_mut_any()
			.downcast_mut::<PortValue<T>>()
		{
			t_ref.set(value);
			Ok(())
		} else {
			Err(crate::error::Error::WrongDataType)
		}
	}

	fn write(&mut self) -> crate::error::Result<PortValueWriteGuard<T>> {
		PortValueWriteGuard::new(self.0.clone())
	}

	fn try_write(&mut self) -> crate::error::Result<PortValueWriteGuard<T>> {
		PortValueWriteGuard::try_new(self.0.clone())
	}
}

impl Clone for BoundInOutPort {
	fn clone(&self) -> Self {
		BoundInOutPort(self.0.clone())
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
