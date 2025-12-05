// Copyright © 2025 Stephan Kunz
//! PortDescription tests: usage of macros with simple copy types

#![allow(unused)]
#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use dataports::{
	PortDescriptionProvider, inout_port_description, input_port_description, output_port_description,
	port_description_provider,
};

const INOUT: &str = "inout";
static OUT: &str = "out";

const DEFAULT_INOUT: u32 = 42;
static DEFAULT_OUT: i32 = -42;

const COMMENT_INOUT: &str = "comment_inout";
static COMMENT_OUT: &str = "comment_out";

struct MacroImplementation {}

port_description_provider!(
	MacroImplementation,
	input_port_description!(i32, "in", 24, "comment_in"),
	inout_port_description!(u32, INOUT, DEFAULT_INOUT, COMMENT_INOUT),
	output_port_description!(i32, OUT, DEFAULT_OUT, COMMENT_OUT),
);

#[test]
fn macro_implementation() {
	let static_provider = MacroImplementation {};
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
