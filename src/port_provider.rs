// Copyright © 2025 Stephan Kunz
//! [`dataports`](crate) [`PortProvider`] implementation.

use crate::{Error, Port, Port::In, Port::InOut, Port::Out};

/// PortProvider.
pub trait PortProvider {
	/// Returns `true` if the [`Port`] with `key` is available, otherwise `false`.
	#[must_use]
    fn has_port(&self, port_name: &str) -> bool {
        if self.portlist().iter().any(|port| {
            match port {
                In{name} | InOut{name} | Out{name} => *name == port_name
            }
        }) {
            return true;
        }

        false
    }

    /// Returns the [`Port`] if available.
    fn port(&self, port_name: &str) -> Option<&Port> {
        self.portlist().iter().find(|port| match port {
            In{name} | InOut{name} | Out{name} => *name == port_name
        })
    }

    /// List of provided [`Port`]'s.
    fn portlist(&self) -> &[Port];
}


/// DynamicPortProvider.
pub trait DynamicPortProvider: PortProvider {
    /// Adds a [`Port`] with an initial value.
    fn add<T>(&mut self, key: &str, value: T) -> Result<(), Error>;

    /// Removes a [`Port`] returning the value.
    fn remove<T>(&mut self, key: &str) -> Result<T, Error>;
}

