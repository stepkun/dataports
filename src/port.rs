// Copyright © 2025 Stephan Kunz
//! [`dataports`](crate) [`Port`] implementation.

use core::str;

use crate::Error;

/// Port base.
pub trait PortBase {
	/// Returns how often the port's value has been updated/changed.
    fn num_updates(&self) -> Result<usize, Error>;
}

/// Input port.
pub trait InPort<T>: PortBase {
    /// Returns a reference to the port's value.
    fn as_ref(&self) -> Result<&T, Error>;

    /// Returns a copy of the ports value.
    fn get(&self) -> Result<T, Error>;
}

/// Output port.
pub trait OutPort<T>: PortBase {
    /// Returns a mutable reference to the port's value.
    fn as_mut(&mut self) -> Result<&mut T, Error>;

    /// Sets the value of the port.
    fn set(&mut self, value: T) -> Result<Option<T>, Error>;
}

/// Output port.
pub trait InOutPort<T>: InPort<T> + OutPort<T> {}

/// Port.
pub enum Port {
    In{name: &'static str},
    InOut{name: &'static str},
    Out{name: &'static str},
}
#[cfg(test)]
mod tests {
	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<Port>();
	}
}
