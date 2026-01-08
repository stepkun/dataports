// Copyright © 2026 Stephan Kunz
//! An extendable sorted collection of ports.

#![allow(unused)]

use alloc::collections::btree_map::BTreeMap;

use crate::{
	ConstString, PortCommons,
	bind::port_value::{PortValueReadGuard, PortValueWriteGuard},
	collections::PortProvider,
	error::Result,
	port::Port,
};

/// An extendable sorted map of [`Port`]s.
#[derive(Default)]
#[repr(transparent)]
pub struct PortMap(BTreeMap<ConstString, Port>);

impl PortProvider for PortMap {
	fn find(&self, name: &str) -> Option<&Port> {
		self.0
			.values()
			.into_iter()
			.find(|port| &*port.name() == name)
			.map(|v| v as _)
	}

	fn find_mut(&mut self, name: &str) -> Option<&mut Port> {
		self.0
			.values_mut()
			.into_iter()
			.find(|port| &*port.name() == name)
			.map(|v| v as _)
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
