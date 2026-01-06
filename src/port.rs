// Copyright © 2026 Stephan Kunz
//! A type erased port.

#![allow(unused)]

use core::any::Any;

use alloc::boxed::Box;

use crate::{
	BoundInOutPort, BoundOutPort, ConstString, FlowingInOutPort, FlowingInPort, FlowingOutPort, PortCommons,
	any_port::AnyPortType, bind::in_port::BoundInPort,
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
