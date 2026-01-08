// Copyright © 2026 Stephan Kunz
//! A type erased port.

#![allow(unused)]

use core::any::Any;

use alloc::boxed::Box;

use crate::{
	ConstString, PortCommons,
	any_port::AnyPortType,
	bind::{
		BindCommons, BindIn, BindInOut, BindOut, in_out_port::BoundInOutPort, in_port::BoundInPort, out_port::BoundOutPort,
	},
	error::Result,
	flow::{in_out_port::FlowingInOutPort, in_port::FlowingInPort, out_port::FlowingOutPort},
};

/// A type erased port.
pub struct Port {
	name: ConstString,
	ty: Box<dyn AnyPortType>,
}

impl Port {
	pub fn new(name: impl Into<ConstString>, port_type: impl AnyPortType) -> Self {
		Self {
			name: name.into(),
			ty: Box::new(port_type),
		}
	}
}

impl core::fmt::Debug for Port {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Port")
			.field("name", &self.name)
			.field("type", &self.ty)
			.finish()
	}
}

impl Clone for Port {
	fn clone(&self) -> Self {
		let ty: Box<dyn AnyPortType> = {
			if let Some(port) = self.ty.as_any().downcast_ref::<BoundInPort>() {
				Box::new(port.clone())
			} else if let Some(port) = self.ty.as_any().downcast_ref::<BoundOutPort>() {
				Box::new(port.clone())
			} else if let Some(port) = self.ty.as_any().downcast_ref::<BoundInOutPort>() {
				Box::new(port.clone())
			} else if let Some(port) = self.ty.as_any().downcast_ref::<FlowingInPort>() {
				Box::new(port.clone())
			} else if let Some(port) = self.ty.as_any().downcast_ref::<FlowingOutPort>() {
				Box::new(port.clone())
			} else if let Some(port) = self
				.ty
				.as_any()
				.downcast_ref::<FlowingInOutPort>()
			{
				Box::new(port.clone())
			} else {
				todo!("missing implementation for new port type")
			}
		};
		Self {
			name: self.name.clone(),
			ty,
		}
	}
}

impl PortCommons for Port {
	fn name(&self) -> ConstString {
		self.name.clone()
	}
}

impl BindCommons for Port {
	fn bind_to(&mut self, other: &dyn AnyPortType) -> Result<()> {
		todo!()
	}
}

impl<T> BindIn<T> for Port {
	fn get(&self) -> Option<T>
	where
		T: Clone,
	{
		todo!()
	}

	fn read(&self) -> crate::error::Result<crate::PortValueReadGuard<T>> {
		todo!()
	}

	fn try_read(&self) -> crate::error::Result<crate::PortValueReadGuard<T>> {
		todo!()
	}
}
impl<T> BindInOut<T> for Port {
	fn replace(&mut self, value: T) -> Option<T> {
		todo!()
	}

	fn take(&mut self) -> Option<T> {
		todo!()
	}
}

impl<T> BindOut<T> for Port {
	fn set(&mut self, value: T) -> Result<()> {
		todo!()
	}

	fn write(&mut self) -> Result<crate::PortValueWriteGuard<T>> {
		todo!()
	}

	fn try_write(&mut self) -> Result<crate::PortValueWriteGuard<T>> {
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
		is_normal::<&Port>();
		is_normal::<Port>();
	}
}
