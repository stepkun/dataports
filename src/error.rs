// Copyright © 2026 Stephan Kunz
//! Error implementation.

use crate::ConstString;

/// Shortcut for [`dataport`](crate)'s Result<T, E> type
pub(crate) type Result<T> = core::result::Result<T, Error>;

/// Port errors.
#[non_exhaustive]
pub enum Error {
	/// A ports value is currently locked.
	IsLocked,
	/// No value set for a port.
	NoValueSet,
	/// A port has other data type then expected.
	WrongDataType,
}

/// Only default implementation needed.
impl core::error::Error for Error {}

impl core::fmt::Debug for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::IsLocked => write!(f, "IsLocked(port"),
			Self::NoValueSet => write!(f, "NoValueSet"),
			Self::WrongDataType => write!(f, "WrongDataType"),
		}
	}
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::IsLocked => write!(f, "port is currently locked"),
			Self::NoValueSet => write!(f, "no value set for port"),
			Self::WrongDataType => write!(f, "port has a different data type then expected"),
		}
	}
}
