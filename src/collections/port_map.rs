// Copyright © 2026 Stephan Kunz
//! An extendable sorted collection of ports.

#![allow(unused)]

use alloc::collections::btree_map::BTreeMap;

use crate::{ConstString, port::Port};

/// An extendable sorted map of [`Port`]s.
#[derive(Default)]
#[repr(transparent)]
struct PortMap(BTreeMap<ConstString, Port>);

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
