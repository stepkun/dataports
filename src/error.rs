// Copyright © 2026 Stephan Kunz
//! Error implementation.

use crate::ConstString;

/// Shortcut for [`dataport`](crate)'s Result<T, E> type
pub(crate) type Result<T> = core::result::Result<T, Error>;

/// Port errors.
#[non_exhaustive]
pub(crate) enum Error {
	/// Ports value is currently locked.
	IsLocked {
		/// Name of the port.
		port: ConstString,
	},
	/// No value set for a port.
	NoValueSet {
		/// Name of the port.
		port: ConstString,
	},
	/// Port has other data type then exoected.
	WrongDataType {
		/// Name of the port.
		port: ConstString,
	},
}

/// Only default implementation needed.
impl core::error::Error for Error {}

impl core::fmt::Debug for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::IsLocked { port } => write!(f, "IsLocked(port: {port})"),
			Self::NoValueSet { port } => write!(f, "NoValueSet(port: {port})"),
			Self::WrongDataType { port } => write!(f, "WrongDataType(port: {port})"),
		}
	}
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::IsLocked { port } => write!(f, "port '{port}' is currently locked"),
			Self::NoValueSet { port } => write!(f, "no value set for port '{port}'"),
			Self::WrongDataType { port } => write!(f, "port '{port}' has a different data type then expected"),
		}
	}
}
