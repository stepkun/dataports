// Copyright © 2026 Stephan Kunz
//! Module implementing port collections.

#![allow(unused)]

use crate::{
	BindCommons, ConstString,
	bind::{
		BindIn, BindInOut, BindOut,
		port_value::{PortValueReadGuard, PortValueWriteGuard},
	},
	error::{Error, Result},
	port_variant::PortVariant,
};

pub mod port_array;
pub mod port_list;
pub mod port_map;

/// Methods for something that provides ports.
/// Each port is identified by its name, so the name has to be unique within a certain port provider.
pub trait PortProvider {
	/// Returns a port from the collection.
	/// This method needs a collection specific implementation.
	fn find(&self, name: &str) -> Option<&PortVariant>;

	/// Returns a mutable port from the collection.
	/// This method needs a collection specific implementation.
	fn find_mut(&mut self, name: &str) -> Option<&mut PortVariant>;

	/// Connects a port from this provider to a port from another provider.
	/// Type of connection depends on types of both ports.
	fn connect_to(&mut self, name: &str, other_provider: &impl PortProvider, other_name: &str) -> Result<()> {
		if let Some(port) = self.find_mut(name) {
			if let Some(other) = other_provider.find(other_name) {
				match port {
					PortVariant::InBound(port) => port.bind_to(other),
					PortVariant::InOutBound(port) => port.bind_to(other),
					PortVariant::OutBound(port) => port.bind_to(other),
				}
			} else {
				Err(Error::NotFound { name: other_name.into() })
			}
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}
}

pub trait PortAccessors<T: 'static + Send + Sync + core::fmt::Debug>: PortProvider {
	/// Returns a clone/copy of the T.
	/// Therefore T must implement [`Clone`].
	fn get(&self, name: &str) -> Result<Option<T>>
	where
		T: Clone,
	{
		if let Some(port) = self.find(name) {
			match port {
				PortVariant::InBound(port) => Ok(port.get()),
				PortVariant::InOutBound(port) => Ok(port.get()),
				PortVariant::OutBound(_) => Err(Error::WrongPortType),
			}
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`](crate::error::Error), if port is not in port list.
	/// - [`Error::WrongDataType`](crate::error::Error), if port is not the expected port type & type of T.
	fn read(&self, name: &str) -> Result<PortValueReadGuard<T>> {
		if let Some(port) = self.find(name) {
			match port {
				PortVariant::InBound(port) => port.read(),
				PortVariant::InOutBound(port) => port.read(),
				PortVariant::OutBound(_) => Err(Error::WrongPortType),
			}
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`](crate::error::Error), if port is locked.
	/// - [`Error::NotFound`](crate::error::Error), if port is not in port list.
	/// - [`Error::WrongDataType`](crate::error::Error), if port is not the expected port type & type of T.
	fn try_read(&self, name: &str) -> Result<PortValueReadGuard<T>> {
		if let Some(port) = self.find(name) {
			match port {
				PortVariant::InBound(port) => port.try_read(),
				PortVariant::InOutBound(port) => port.try_read(),
				PortVariant::OutBound(_) => Err(Error::WrongPortType),
			}
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}

	/// Sets a new value to the T and returns the old T.
	fn replace(&mut self, name: &str, value: T) -> Result<Option<T>> {
		if let Some(port) = self.find_mut(name) {
			match port {
				PortVariant::InOutBound(port) => Ok(port.replace(value)),
				PortVariant::InBound(_) | PortVariant::OutBound(_) => Err(Error::WrongPortType),
			}
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}

	/// Returns the T, removing it from the port.
	fn take(&mut self, name: &str) -> Result<Option<T>> {
		if let Some(port) = self.find_mut(name) {
			match port {
				PortVariant::InOutBound(port) => Ok(port.take()),
				PortVariant::InBound(_) | PortVariant::OutBound(_) => Err(Error::WrongPortType),
			}
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}

	/// Sets a new value to the T.
	fn set(&mut self, name: &str, value: T) -> Result<()> {
		if let Some(port) = self.find_mut(name) {
			match port {
				PortVariant::OutBound(port) => port.set(value),
				PortVariant::InOutBound(port) => port.set(value),
				PortVariant::InBound(_) => Err(Error::WrongPortType),
			}
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`](crate::error::Error), if port is not in port list.
	/// - [`Error::WrongDataType`](crate::error::Error), if port is not the expected port type & type of T.
	fn write(&mut self, name: &str) -> Result<PortValueWriteGuard<T>> {
		if let Some(port) = self.find_mut(name) {
			match port {
				PortVariant::OutBound(port) => port.write(),
				PortVariant::InOutBound(port) => port.write(),
				PortVariant::InBound(_) => Err(Error::WrongPortType),
			}
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`](crate::error::Error), if port is locked.
	/// - [`Error::NotFound`](crate::error::Error), if port is not in port list.
	/// - [`Error::WrongDataType`](crate::error::Error), if port is not the expected port type & type of T.
	fn try_write(&mut self, name: &str) -> Result<PortValueWriteGuard<T>> {
		if let Some(port) = self.find_mut(name) {
			match port {
				PortVariant::OutBound(port) => port.try_write(),
				PortVariant::InOutBound(port) => port.try_write(),
				PortVariant::InBound(_) => Err(Error::WrongPortType),
			}
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}
}
