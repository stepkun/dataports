// Copyright © 2026 Stephan Kunz
//! An extendable unsorted collection of ports.

#![allow(unused)]

use alloc::vec::Vec;

use crate::{
	PortCommons,
	bind::port_value::{PortValueReadGuard, PortValueWriteGuard},
	collections::PortProvider,
	error::{Error, Result},
	port::Port,
};

/// An extendable unsorted list of [`Port`]s.
#[derive(Default)]
#[repr(transparent)]
pub struct PortList(Vec<Port>);

impl PortList {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn add(&mut self, port: Port) -> Result<()> {
		let name = port.name();
		if self.find(&name).is_some() {
			Err(Error::AlreadyInCollection { name })
		} else {
			self.0.push(port);
			Ok(())
		}
	}
}

impl PortProvider for PortList {
	fn find(&self, name: &str) -> Option<&Port> {
		self.0
			.iter()
			.find(|port| &*port.name() == name)
			.map(|v| v as _)
	}

	fn find_mut(&mut self, name: &str) -> Option<&mut Port> {
		self.0
			.iter_mut()
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
		is_normal::<&PortList>();
		is_normal::<PortList>();
	}
}
