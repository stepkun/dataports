// Copyright © 2025 Stephan Kunz
//! [`dataports`](crate) errors.

/// Port errors.
#[non_exhaustive]
pub enum Error {
}

/// Only default implementation needed.
impl core::error::Error for Error {}

impl core::fmt::Debug for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			_ => write!(f, "Unknown"),
		}
	}
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			_ => write!(f, "Unknown error in crate [dataports]"),
		}
	}
}
