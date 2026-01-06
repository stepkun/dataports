// Copyright © 2026 Stephan Kunz
//! An extendable unsorted collection of ports.

#![allow(unused)]

use alloc::vec::Vec;

use crate::port::Port;

/// An extendable unsorted list of [`Port`]s.
#[derive(Default)]
#[repr(transparent)]
struct PortList(Vec<Port>);

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
