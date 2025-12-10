// Copyright © 2025 Stephan Kunz
//! PortDescription tests: manual & usage of macro `port_description_provider!`

#![allow(unused)]
#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use dataports::{
	PortDescription, PortDescriptionProvider, port_description::PortDescriptionData, port_description_provider,
};

const INOUT: &str = "inout";
static OUT: &str = "out";

const DEFAULT_INOUT: &str = "default_inout";
static DEFAULT_OUT: &str = "default_out";

const COMMENT_INOUT: &str = "comment_inout";
static COMMENT_OUT: &str = "comment_out";

struct ManualImplementation {}

static MANUAL_LIST: [PortDescription; 3] = [
	PortDescription::In(PortDescriptionData::new("in", None, None)),
	PortDescription::InOut(PortDescriptionData::new(INOUT, None, None)),
	PortDescription::Out(PortDescriptionData::new(OUT, None, None)),
];

impl PortDescriptionProvider for ManualImplementation {
	fn port_description_list(&self) -> &[dataports::PortDescription] {
		&MANUAL_LIST
	}
}

#[test]
fn ports_without_manual() {
	let static_provider = ManualImplementation {};
	assert!(static_provider.has_port("in"));
	assert!(
		static_provider
			.port_description("in")
			.unwrap()
			.default()
			.is_none()
	);
	assert!(static_provider.has_port("inout"));
	assert!(
		static_provider
			.port_description("inout")
			.unwrap()
			.default()
			.is_none()
	);
	assert!(static_provider.has_port(INOUT));
	assert!(static_provider.has_port("out"));
	assert!(
		static_provider
			.port_description("out")
			.unwrap()
			.default()
			.is_none()
	);
	assert!(static_provider.has_port(OUT));
}

struct MacroImplementation {}

port_description_provider!(
	MacroImplementation,
	PortDescription::In(PortDescriptionData::new("in", None, None)),
	PortDescription::InOut(PortDescriptionData::new(INOUT, None, None)),
	PortDescription::Out(PortDescriptionData::new(OUT, None, None)),
);

#[test]
fn ports_without_macro() {
	let static_provider = MacroImplementation {};
	assert!(static_provider.has_port("in"));
	assert!(
		static_provider
			.port_description("in")
			.unwrap()
			.default()
			.is_none()
	);
	assert!(
		static_provider
			.port_description("in")
			.unwrap()
			.comment()
			.is_none()
	);
	assert!(static_provider.has_port("inout"));
	assert!(
		static_provider
			.port_description("inout")
			.unwrap()
			.default()
			.is_none()
	);
	assert!(
		static_provider
			.port_description("inout")
			.unwrap()
			.comment()
			.is_none()
	);
	assert!(static_provider.has_port(INOUT));
	assert!(static_provider.has_port("out"));
	assert!(
		static_provider
			.port_description("out")
			.unwrap()
			.default()
			.is_none()
	);
	assert!(
		static_provider
			.port_description("out")
			.unwrap()
			.comment()
			.is_none()
	);
	assert!(static_provider.has_port(OUT));
}
