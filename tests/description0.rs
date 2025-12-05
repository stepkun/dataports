// Copyright © 2025 Stephan Kunz
//! PortDescription tests: no ports

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use dataports::{PortDescriptionProvider, port_description_provider};

struct NoImplementation {}

port_description_provider!(NoImplementation);

#[test]
fn no_implementation() {
	let static_provider = NoImplementation {};
	assert!(!static_provider.has_port("in"));
	assert!(static_provider.port_description_list().is_empty());
	assert!(static_provider.port_description("in").is_none());
}
