// Copyright © 2026 Stephan Kunz
//! A bound output type port implementing [`BindOut`].

use alloc::{boxed::Box, sync::Arc};
use spin::RwLock;

use crate::{
	bind::{
		BindCommons, BindOut,
		any_port_value::AnyPortValueType,
		in_out_port::BoundInOutPort,
		in_port::BoundInPort,
		port_value::{PortValue, PortValuePtr, PortValueWriteGuard},
	},
	error::{Error, Result},
};

/// @TODO:
#[derive(Debug)]
pub struct BoundOutPort(PortValuePtr);

impl BoundOutPort {
	pub fn empty<T: AnyPortValueType>() -> Self {
		Self(Arc::new(RwLock::new(PortValue::<T>::empty())))
	}

	pub fn new<T: AnyPortValueType>(value: T) -> Self {
		Self(Arc::new(RwLock::new(PortValue::<T>::new(value))))
	}

	pub(crate) fn set_value(&mut self, value: PortValuePtr) -> Result<()> {
		let x = (*self.0.read()).type_id();
		let y = (*value.read()).type_id();
		if x == y {
			self.0 = value;
			Ok(())
		} else {
			Err(Error::WrongDataType)
		}
	}

	pub(crate) fn value(&self) -> PortValuePtr {
		self.0.clone()
	}
}

impl BindCommons for BoundOutPort {
	fn bind(&mut self, other: &dyn crate::any_port::AnyPortType) -> Result<()> {
		if let Some(port) = other.as_any().downcast_ref::<BoundInPort>() {
			self.set_value(port.value())
		} else if let Some(port) = other.as_any().downcast_ref::<BoundOutPort>() {
			self.set_value(port.value())
		} else if let Some(port) = other.as_any().downcast_ref::<BoundInOutPort>() {
			self.set_value(port.value())
		} else {
			todo!("missing implementation for new port type")
		}
	}
}

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
