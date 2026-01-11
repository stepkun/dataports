// Copyright © 2026 Stephan Kunz
//! An extendable unsorted collection of ports.

#![allow(unused)]

use alloc::vec::Vec;

use crate::{
	ConstString,
	bind::{
		any_port_value::AnyPortValueType,
		port_value::{PortValueReadGuard, PortValueWriteGuard},
	},
	collections::{PortAccessors, PortProvider},
	error::{Error, Result},
	port_variant::PortVariant,
};

/// An extendable unsorted list of [`Port`]s.
#[derive(Default)]
#[repr(transparent)]
pub struct PortList(Vec<(ConstString, PortVariant)>);

impl PortList {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn add(&mut self, name: impl Into<ConstString>, port: PortVariant) -> Result<()> {
		let name = name.into();
		if self.find(&name).is_some() {
			Err(Error::AlreadyInCollection { name })
		} else {
			self.0.push((name, port));
			Ok(())
		}
	}
}

impl PortProvider for PortList {
	fn find(&self, name: &str) -> Option<&PortVariant> {
		self.0
			.iter()
			.find(|port| &*port.0 == name)
			.map(|v| &v.1 as _)
	}

	fn find_mut(&mut self, name: &str) -> Option<&mut PortVariant> {
		self.0
			.iter_mut()
			.find(|port| &*port.0 == name)
			.map(|v| &mut v.1 as _)
	}
}

impl PortAccessors for PortList {}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&PortList>();
		is_normal::<PortList>();
	}
}
