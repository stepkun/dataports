// Copyright © 2025 Stephan Kunz
//! Definition tests for dataports: fully manual and usages of macros

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use dataports::{
	PortDescription, PortDescriptionProvider, inout_port_description, input_port_description, output_port_description,
	port_description_list, port_description_provider,
};

const INOUT: &str = "inout";
static OUT: &str = "out";

const DESC_INOUT: &str = "desc_inout";
static DESC_OUT: &str = "desc_out";

struct ManualImplementation {}

static MANUAL_LIST: [PortDescription; 3] = [
	PortDescription::In {
		name: "in",
		description: None,
	},
	PortDescription::InOut {
		name: INOUT,
		description: None,
	},
	PortDescription::Out {
		name: OUT,
		description: None,
	},
];

impl PortDescriptionProvider for ManualImplementation {
	fn port_description_list(&self) -> &[dataports::PortDescription] {
		&MANUAL_LIST
	}
}

#[test]
fn manual_implementation() {
	let static_provider = ManualImplementation {};
	assert!(static_provider.has_port("in"));
	assert!(static_provider.has_port("inout"));
	assert!(static_provider.has_port(INOUT));
	assert!(static_provider.has_port("out"));
	assert!(static_provider.has_port(OUT));
}

struct MixedImplementation {}

static GENERATED_LIST: [PortDescription; 3] = port_description_list![
	input_port_description!("in", "desc_in"),
	inout_port_description!(INOUT, DESC_INOUT),
	output_port_description!(OUT, DESC_OUT),
];

impl PortDescriptionProvider for MixedImplementation {
	fn port_description_list(&self) -> &[dataports::PortDescription] {
		&GENERATED_LIST
	}
}

#[test]
fn mixed_implementation() {
	let static_provider = MixedImplementation {};
	assert!(static_provider.has_port("in"));
	assert_eq!(
		static_provider
			.port_description("in")
			.unwrap()
			.description()
			.unwrap(),
		"desc_in"
	);
	assert!(static_provider.has_port("inout"));
	assert!(static_provider.has_port(INOUT));
	assert_eq!(
		static_provider
			.port_description("inout")
			.unwrap()
			.description()
			.unwrap(),
		"desc_inout"
	);
	assert_eq!(
		static_provider
			.port_description(INOUT)
			.unwrap()
			.description()
			.unwrap(),
		DESC_INOUT
	);
	assert!(static_provider.has_port("out"));
	assert!(static_provider.has_port(OUT));
	assert_eq!(
		static_provider
			.port_description("out")
			.unwrap()
			.description()
			.unwrap(),
		"desc_out"
	);
	assert_eq!(
		static_provider
			.port_description(OUT)
			.unwrap()
			.description()
			.unwrap(),
		DESC_OUT
	);
}

struct MacroImplementation {}

port_description_provider!(
	MacroImplementation,
	input_port_description!("in"),
	inout_port_description!(INOUT),
	output_port_description!(OUT),
);

#[test]
fn macro_implementation() {
	let static_provider = MacroImplementation {};
	assert!(static_provider.has_port("in"));
	assert!(static_provider.has_port("inout"));
	assert!(static_provider.has_port(INOUT));
	assert!(static_provider.has_port("out"));
	assert!(static_provider.has_port(OUT));
}
