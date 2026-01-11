// Copyright © 2026 Stephan Kunz
//! An extendable sorted collection of ports.

#![allow(unused)]

use alloc::collections::btree_map::BTreeMap;

use crate::{
	ConstString,
	bind::port_value::{PortValueReadGuard, PortValueWriteGuard},
	collections::PortProvider,
	error::{Error, Result},
	port_variant::PortVariant,
};

/// An extendable sorted map of [`Port`]s.
#[derive(Default)]
#[repr(transparent)]
pub struct PortMap(BTreeMap<ConstString, PortVariant>);

impl PortMap {
	pub fn new() -> Self {
		Self(BTreeMap::new())
	}

	pub fn add(&mut self, name: impl Into<ConstString>, port: PortVariant) -> Result<()> {
		let name = name.into();
		if self.find(&name).is_some() {
			Err(Error::AlreadyInCollection { name })
		} else {
			self.0.insert(name, port);
			Ok(())
		}
	}
}

impl PortProvider for PortMap {
	fn find(&self, name: &str) -> Option<&PortVariant> {
		self.0.get(name)
	}

	fn find_mut(&mut self, name: &str) -> Option<&mut PortVariant> {
		self.0.get_mut(name)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&PortMap>();
		is_normal::<PortMap>();
	}
}
