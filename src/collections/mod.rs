// Copyright © 2026 Stephan Kunz
//! Module implementing port collections.

#![allow(unused)]

use crate::{
	ConstString,
	bind::port_value::{PortValueReadGuard, PortValueWriteGuard},
	error::Result,
	port_variant::PortVariant,
};

pub mod port_array;
pub mod port_list;
pub mod port_map;

trait PortCommons {
	fn name(&self) -> ConstString;
}

/// Methods for something that provides ports.
/// Each port is identified by its name, so the name has to be unique within a certain port provider.
pub trait PortProvider {
	/// Returns a port from the collection.
	/// This method needs a collection specific implementation.
	fn find(&self, name: &str) -> Option<&PortVariant>;

	/// Returns a mutable port from the collection.
	/// This method needs a collection specific implementation.
	fn find_mut(&mut self, name: &str) -> Option<&mut PortVariant>;

	/// Connects a port to a port from another list.
	/// Type of connection depends on types of both ports.
	fn connect(&mut self, name: &str, other_list: &impl PortProvider, other_name: &str) -> Result<()> {
		todo!()
	}

	/// Returns a clone/copy of the T.
	/// Therefore T must implement [`Clone`].
	#[must_use]
	fn get<T>(&self, name: &str) -> Option<T>
	where
		T: Clone,
	{
		todo!()
	}

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`](crate::error::Error), if port is not in port list.
	/// - [`Error::WrongDataType`](crate::error::Error), if port is not the expected port type & type of T.
	fn read<T>(&self, name: &str) -> Result<PortValueReadGuard<T>> {
		todo!()
	}

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`](crate::error::Error), if port is locked.
	/// - [`Error::NotFound`](crate::error::Error), if port is not in port list.
	/// - [`Error::WrongDataType`](crate::error::Error), if port is not the expected port type & type of T.
	fn try_read<T>(&self, name: &str) -> Result<PortValueReadGuard<T>> {
		todo!()
	}

	/// Sets a new value to the T and returns the old T.
	#[must_use]
	fn replace<T>(&mut self, name: &str, value: T) -> Option<T> {
		todo!()
	}

	/// Returns the T, removing it from the port.
	#[must_use]
	fn take<T>(&mut self, name: &str) -> Option<T> {
		todo!()
	}

	/// Sets a new value to the T.
	fn set<T>(&mut self, name: &str, value: T) -> Result<()> {
		todo!()
	}

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`](crate::error::Error), if port is not in port list.
	/// - [`Error::WrongDataType`](crate::error::Error), if port is not the expected port type & type of T.
	fn write<T>(&mut self, name: &str) -> Result<PortValueWriteGuard<T>> {
		todo!()
	}

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`](crate::error::Error), if port is locked.
	/// - [`Error::NotFound`](crate::error::Error), if port is not in port list.
	/// - [`Error::WrongDataType`](crate::error::Error), if port is not the expected port type & type of T.
	fn try_write<T>(&mut self, name: &str) -> Result<PortValueWriteGuard<T>> {
		todo!()
	}
}
