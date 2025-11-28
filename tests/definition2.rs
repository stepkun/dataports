// Copyright © 2025 Stephan Kunz
//! Definition test for dataports: using same name in other file

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use dataports::{PortProvider, inout_port, input_port, output_port, port_provider};

const INOUT: &str = "inout";
const OUT: &str = "out";

struct MacroImplementation {}

port_provider!(MacroImplementation,
    input_port!("in"),
    inout_port!(INOUT),
    output_port!(OUT),
);

#[test]
fn macro_implementation() {
    let static_provider = MacroImplementation{};
    assert!(static_provider.has_port("in"));
    assert!(static_provider.has_port("inout"));
    assert!(static_provider.has_port(INOUT));
    assert!(static_provider.has_port("out"));
    assert!(static_provider.has_port(OUT));
}
