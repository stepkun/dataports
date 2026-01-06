// Copyright © 2026 Stephan Kunz
//! Abstract port value type trait.

#![allow(unused)]

use core::{
	any::Any,
	ops::{Deref, DerefMut},
};

use alloc::sync::Arc;

use crate::{
	ConstString, RwLock, RwLockReadGuard, RwLockWriteGuard,
	bind::sequence_number::SequenceNumber,
	error::{Error, Result},
};

/// The `AnyPortValueType` trait allows to use different types of values in ports.
pub trait AnyPortValueType: Any + Send + Sync + core::fmt::Debug {
	/// Convert to Any
	#[must_use]
	fn as_any(&self) -> &dyn Any;

	/// Convert to mut Any
	#[must_use]
	fn as_mut_any(&mut self) -> &mut dyn Any;
}

macro_rules! implement_any_port_value_type {
	($($tp:ty),*) => {
		$(impl AnyPortValueType for $tp {
			fn as_any(&self) -> &dyn Any {
				self
			}
			fn as_mut_any(&mut self) -> &mut dyn Any {
				self
			}
		})*
	};
}

implement_any_port_value_type!(bool, usize, isize);
implement_any_port_value_type!(u128, i128, u64, i64, u32, i32, u16, i16, u8, i8);
implement_any_port_value_type!(f64, f32);
implement_any_port_value_type!(&'static str);
implement_any_port_value_type!(::alloc::string::String);

impl<T: 'static + Send + Sync + core::fmt::Debug> AnyPortValueType for ::alloc::vec::Vec<T> {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_mut_any(&mut self) -> &mut dyn Any {
		self
	}
}

impl<K: 'static + Send + Sync + core::fmt::Debug, V: 'static + Send + Sync + core::fmt::Debug> AnyPortValueType
	for ::alloc::collections::BTreeMap<K, V>
{
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_mut_any(&mut self) -> &mut dyn Any {
		self
	}
}

impl<T: 'static + Send + Sync + core::fmt::Debug> AnyPortValueType for ::alloc::collections::BTreeSet<T> {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_mut_any(&mut self) -> &mut dyn Any {
		self
	}
}
