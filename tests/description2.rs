// Copyright © 2025 Stephan Kunz
//! Definition test for dataports: using same name in other file

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use dataports::{
	PortDescriptionProvider, inout_port_description, input_port_description, output_port_description,
	port_description_provider,
};

const INOUT: &str = "inout";
static OUT: &str = "out";

const DESC_INOUT: &str = "desc_inout";
static DESC_OUT: &str = "desc_out";

struct MacroImplementation {}

port_description_provider!(
	MacroImplementation,
	input_port_description!("in", "desc_in"),
	inout_port_description!(INOUT, DESC_INOUT),
	output_port_description!(OUT, DESC_OUT),
);

#[test]
fn macro_implementation() {
	let static_provider = MacroImplementation {};
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
