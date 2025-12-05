// Copyright © 2025 Stephan Kunz
//! PortDescription tests: fully manual, mixed usage and only usage of macros with borrowed type

#![allow(unused)]
#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use dataports::{
	PortDescription, PortDescriptionProvider, inout_port_description, input_port_description, output_port_description,
	port_description::PortDescriptionData, port_description_list, port_description_provider,
};

const INOUT: &str = "inout";
static OUT: &str = "out";

const DEFAULT_INOUT: &str = "default_inout";
static DEFAULT_OUT: &str = "default_out";

const COMMENT_INOUT: &str = "comment_inout";
static COMMENT_OUT: &str = "comment_out";

struct ManualImplementation {}

static MANUAL_LIST: [PortDescription; 3] = [
	PortDescription::In(PortDescriptionData::new("&'static str", "in", None, None)),
	PortDescription::InOut(PortDescriptionData::new("&'static str", INOUT, None, None)),
	PortDescription::Out(PortDescriptionData::new("&'static str", OUT, None, None)),
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
	input_port_description!(&'static str, "in", "default_in", "comment_in"),
	inout_port_description!(&'static str, INOUT, DEFAULT_INOUT, COMMENT_INOUT),
	output_port_description!(&'static str, OUT, DEFAULT_OUT, COMMENT_OUT),
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
			.comment()
			.unwrap(),
		"comment_in"
	);
	assert!(static_provider.has_port("inout"));
	assert!(static_provider.has_port(INOUT));
	assert_eq!(
		static_provider
			.port_description("inout")
			.unwrap()
			.comment()
			.unwrap(),
		"comment_inout"
	);
	assert_eq!(
		static_provider
			.port_description(INOUT)
			.unwrap()
			.comment()
			.unwrap(),
		COMMENT_INOUT
	);
	assert!(static_provider.has_port("out"));
	assert!(static_provider.has_port(OUT));
	assert_eq!(
		static_provider
			.port_description("out")
			.unwrap()
			.comment()
			.unwrap(),
		"comment_out"
	);
	assert_eq!(
		static_provider
			.port_description(OUT)
			.unwrap()
			.comment()
			.unwrap(),
		COMMENT_OUT
	);
}

struct MacroImplementation {}

port_description_provider!(
	MacroImplementation,
	input_port_description!(str, "in"),
	inout_port_description!(str, INOUT),
	output_port_description!(str, OUT),
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
