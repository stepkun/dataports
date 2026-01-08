// Copyright © 2026 Stephan Kunz
//! A bound input type port implementing [`BindIn`].

use core::any::Any;

use alloc::{boxed::Box, sync::Arc};
use spin::RwLock;

use crate::{
	bind::{
		BindCommons, BindIn,
		any_port_value::AnyPortValueType,
		in_out_port::BoundInOutPort,
		out_port::BoundOutPort,
		port_value::{PortValue, PortValuePtr, PortValueReadGuard},
	},
	error::{Error, Result},
};

/// @TODO:
#[derive(Debug)]
pub struct BoundInPort(PortValuePtr);

impl BoundInPort {
	pub fn new<T: AnyPortValueType>() -> Self {
		Self(Arc::new(RwLock::new(PortValue::<T>::empty())))
	}

	pub fn with_value<T: AnyPortValueType>(value: T) -> Self {
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

impl BindCommons for BoundInPort {
	fn bind_to(&mut self, other: &dyn crate::any_port::AnyPortType) -> crate::error::Result<()> {
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

impl<T: AnyPortValueType> BindIn<T> for BoundInPort {
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

impl Clone for BoundInPort {
	fn clone(&self) -> Self {
		BoundInPort(self.0.clone())
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
