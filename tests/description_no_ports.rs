// Copyright © 2025 Stephan Kunz
//! PortDescription tests: no ports

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use dataports::{PortDescription, PortDescriptionProvider, port_description_provider};

struct ManualImplementation;

static MANUAL_LIST: [PortDescription; 0] = [];

impl PortDescriptionProvider for ManualImplementation {
	fn port_description_list(&self) -> &[dataports::PortDescription] {
		&MANUAL_LIST
	}
}

#[test]
fn no_ports_manual() {
	let static_provider = ManualImplementation;
	assert!(!static_provider.has_port("in"));
	assert!(static_provider.port_description_list().is_empty());
	assert!(static_provider.port_description("in").is_none());
}

struct MacroImplementation;

port_description_provider!(MacroImplementation);

#[test]
fn no_ports_macro() {
	let static_provider = MacroImplementation;
	assert!(!static_provider.has_port("in"));
	assert!(static_provider.port_description_list().is_empty());
	assert!(static_provider.port_description("in").is_none());
}
