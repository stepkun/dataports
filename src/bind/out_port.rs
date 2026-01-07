// Copyright © 2026 Stephan Kunz
//! A bound output type port implementing [`BindOut`].

use alloc::{boxed::Box, sync::Arc};
use spin::RwLock;

use crate::{
	bind::{
		BindCommons, BindOut,
		any_port_value::AnyPortValueType,
		port_value::{PortValue, PortValuePtr, PortValueWriteGuard},
	},
	error::{Error, Result},
};

/// @TODO:
#[derive(Debug)]
pub struct BoundOutPort(PortValuePtr);

impl BoundOutPort {
	pub fn new<T: AnyPortValueType>(value: T) -> Self {
		Self(Arc::new(RwLock::new(PortValue::<T>::new(value))))
	}
}

impl BindCommons for BoundOutPort {}

impl<T: AnyPortValueType> BindOut<T> for BoundOutPort {
	fn set(&mut self, value: T) -> Result<()> {
		let mut any_value = &mut *self.0.write();
		if let Some(t_ref) = any_value
			.as_mut_any()
			.downcast_mut::<PortValue<T>>()
		{
			t_ref.set(value);
			Ok(())
		} else {
			Err(Error::WrongDataType)
		}
	}

	fn write(&mut self) -> crate::error::Result<PortValueWriteGuard<T>> {
		PortValueWriteGuard::new(self.0.clone())
	}

	fn try_write(&mut self) -> crate::error::Result<PortValueWriteGuard<T>> {
		PortValueWriteGuard::try_new(self.0.clone())
	}
}

impl Clone for BoundOutPort {
	fn clone(&self) -> Self {
		BoundOutPort(self.0.clone())
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
