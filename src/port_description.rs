// Copyright © 2025 Stephan Kunz
//! [`dataports`](crate) [`PortDescription`] implementation.

use core::{ops::Deref, str};

/// PortDescriptionProvider.
pub trait PortDescriptionProvider {
	/// Returns `true` if the [`PortDescription`] for `port_name` is available, otherwise `false`.
	#[must_use]
	fn has_port(&self, port_name: &str) -> bool {
		if self
			.port_description_list()
			.iter()
			.any(|description| description.deref().name == port_name)
		{
			return true;
		}

		false
	}

	/// Returns the [`PortDescription`] for `port_name` if available.
	#[must_use]
	fn port_description(&self, port_name: &str) -> Option<&PortDescription> {
		self.port_description_list()
			.iter()
			.find(|description| (*description).deref().name == port_name)
	}

	/// List of provided [`Port`]'s.
	#[must_use]
	fn port_description_list(&self) -> &[PortDescription];
}

/// Inner data structure for the enum [`PortDescription`].
pub struct PortDescriptionData {
	/// Identifying name of the port.
	name: &'static str,
	/// Optional default value.
	default: Option<&'static str>,
	/// Optional comment about the ports purpose.
	comment: Option<&'static str>,
}

impl PortDescriptionData {
	pub const fn new(name: &'static str, default: Option<&'static str>, comment: Option<&'static str>) -> Self {
		Self { name, default, comment }
	}
}

/// Directional PortDescription.
pub enum PortDescription {
	In(PortDescriptionData),
	InOut(PortDescriptionData),
	Out(PortDescriptionData),
}

impl Deref for PortDescription {
	type Target = PortDescriptionData;

	fn deref(&self) -> &Self::Target {
		match self {
			crate::PortDescription::In(inner)
			| crate::PortDescription::InOut(inner)
			| crate::PortDescription::Out(inner) => inner,
		}
	}
}

impl PortDescription {
	pub fn comment(&self) -> Option<&'static str> {
		self.deref().comment
	}

	pub fn default(&self) -> Option<&'static str> {
		self.deref().default
	}

	pub fn name(&self) -> &'static str {
		self.deref().name
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<&PortDescription>();
		is_normal::<PortDescription>();
	}
}
