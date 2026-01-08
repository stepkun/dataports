// Copyright © 2026 Stephan Kunz
//! A fixed unsorted collection of ports.

#![allow(unused)]

use crate::{
	PortCommons,
	bind::port_value::{PortValueReadGuard, PortValueWriteGuard},
	collections::PortProvider,
	error::Result,
	port::Port,
};

/// A fixed unsorted array of [`Port`]s.
#[repr(transparent)]
pub struct PortArray<const S: usize>([Port; S]);

impl<const S: usize> PortArray<S> {
	pub fn new(ports: [Port; S]) -> Self {
		Self(ports)
	}
}

impl<const S: usize> core::fmt::Debug for PortArray<S> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_tuple("PortArray").field(&self.0).finish()
	}
}

impl<const S: usize> core::ops::Deref for PortArray<S> {
	type Target = [Port];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<const S: usize> PortProvider for PortArray<S> {
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
		is_normal::<&PortArray<2>>();
		is_normal::<PortArray<4>>();
	}
}
