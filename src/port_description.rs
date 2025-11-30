// Copyright © 2025 Stephan Kunz
//! [`dataports`](crate) [`PortDescription`] implementation.

use core::str;

/// PortDescriptionProvider.
pub trait PortDescriptionProvider {
	/// Returns `true` if the [`PortDescription`] for `port_name` is available, otherwise `false`.
	#[must_use]
	fn has_port(&self, port_name: &str) -> bool {
		if self
			.port_description_list()
			.iter()
			.any(|description| match description {
				crate::PortDescription::In { name, .. }
				| crate::PortDescription::InOut { name, .. }
				| crate::PortDescription::Out { name, .. } => *name == port_name,
			}) {
			return true;
		}

		false
	}

	/// Returns the [`PortDescription`] for `port_name` if available.
	#[must_use]
	fn port_description(&self, port_name: &str) -> Option<&PortDescription> {
		self.port_description_list()
			.iter()
			.find(|description| match description {
				crate::PortDescription::In { name, .. }
				| crate::PortDescription::InOut { name, .. }
				| crate::PortDescription::Out { name, .. } => *name == port_name,
			})
	}

	/// List of provided [`Port`]'s.
	#[must_use]
	fn port_description_list(&self) -> &[PortDescription];
}

/// PortDescription.
pub enum PortDescription {
	In {
		name: &'static str,
		description: Option<&'static str>,
	},
	InOut {
		name: &'static str,
		description: Option<&'static str>,
	},
	Out {
		name: &'static str,
		description: Option<&'static str>,
	},
}

impl PortDescription {
	pub fn description(&self) -> Option<&'static str> {
		match self {
			crate::PortDescription::In { description, .. }
			| crate::PortDescription::InOut { description, .. }
			| crate::PortDescription::Out { description, .. } => *description,
		}
	}

	pub fn name(&self) -> &'static str {
		match self {
			crate::PortDescription::In { name, .. }
			| crate::PortDescription::InOut { name, .. }
			| crate::PortDescription::Out { name, .. } => name,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<PortDescription>();
	}
}
