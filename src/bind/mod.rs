// Copyright © 2026 Stephan Kunz
//! Module containing 'bind' type ports.

#![allow(unused)]

mod any_port_value;
pub mod in_out_port;
pub mod in_port;
pub mod out_port;
mod port_value;
mod sequence_number;

use crate::{
	bind::port_value::{PortValueReadGuard, PortValueWriteGuard},
	error::Result,
};

/// Trait for bind port types.
pub(crate) trait BindCommons {}

/// Trait for incoming bind port types.
pub(crate) trait BindIn: BindCommons {
	/// Returns a clone/copy of the T.
	/// Therefore T must implement [`Clone`].
	#[must_use]
	fn get<T>(&self) -> Option<T>
	where
		T: Clone;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn read<T>(&self) -> Result<PortValueReadGuard<T>>;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn try_read<T>(&self) -> Result<PortValueReadGuard<T>>;
}

/// Trait for incoming and outgoing bind port types.
trait BindInOut: BindIn + BindOut {
	/// Sets a new value to the T and returns the old T.
	#[must_use]
	fn replace<T>(&mut self, value: impl Into<T>) -> Option<T>;

	/// Returns the T, removing it from the port.
	#[must_use]
	fn take<T>(&mut self) -> Option<T>;
}

/// Trait for outgoing bind port types.
trait BindOut: BindCommons {
	/// Sets a new value to the T.
	fn set<T>(&mut self, value: impl Into<T>);

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn write<T>(&mut self) -> Result<PortValueWriteGuard<T>>;

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn try_write<T>(&mut self) -> Result<PortValueWriteGuard<T>>;
}
