// Copyright © 2026 Stephan Kunz
//! A bound input/output type port implementing [`BindIn`], [`BindOut`] and [`BindInOut`].

use alloc::{boxed::Box, sync::Arc};
use spin::RwLock;

use crate::{
	bind::{
		BindCommons, BindIn, BindInOut, BindOut,
		any_port_value::AnyPortValueType,
		in_port::BoundInPort,
		out_port::BoundOutPort,
		port_value::{PortValue, PortValuePtr, PortValueReadGuard, PortValueWriteGuard},
	},
	error::{Error, Result},
	port_variant::PortVariant,
};

/// @TODO:
#[derive(Debug)]
pub struct BoundInOutPort(PortValuePtr);

impl BoundInOutPort {
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

impl BindCommons for BoundInOutPort {
	fn bind_to(&mut self, other: &PortVariant) -> Result<()> {
		match other {
			PortVariant::InBound(port) => self.set_value(port.value()),
			PortVariant::InOutBound(port) => self.set_value(port.value()),
			PortVariant::OutBound(port) => self.set_value(port.value()),
		}
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

	fn read(&self) -> Result<PortValueReadGuard<T>> {
		PortValueReadGuard::new(self.0.clone())
	}

	fn try_read(&self) -> Result<PortValueReadGuard<T>> {
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
			Err(Error::WrongDataType)
		}
	}

	fn write(&mut self) -> Result<PortValueWriteGuard<T>> {
		PortValueWriteGuard::new(self.0.clone())
	}

	fn try_write(&mut self) -> Result<PortValueWriteGuard<T>> {
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
