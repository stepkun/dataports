// Copyright © 2025 Stephan Kunz
//! Definition tests for dataports: manual and usage of macros

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use dataports::{Port, PortProvider, inout_port, input_port, output_port, port_list, port_provider};

const INOUT: &str = "inout";
const OUT: &str = "out";

struct ManualImplementation {}

static MANUAL_LIST: [Port; 3] = [Port::In{name: "in"}, Port::InOut{name: INOUT}, Port::Out{name: OUT}];
impl PortProvider for ManualImplementation {
    fn portlist(&self) -> &[dataports::Port] {
        &MANUAL_LIST
    }
}

#[test]
fn manual_implementation() {
    let static_provider = ManualImplementation{};
    assert!(static_provider.has_port("in"));
    assert!(static_provider.has_port("inout"));
    assert!(static_provider.has_port(INOUT));
    assert!(static_provider.has_port("out"));
    assert!(static_provider.has_port(OUT));
}

struct MixedImplementation {}

static GENERATED_LIST: [Port; 3] = port_list![
    input_port!("in"),
    inout_port!(INOUT),
    output_port!(OUT),
];
impl PortProvider for MixedImplementation {
    fn portlist(&self) -> &[dataports::Port] {
        &GENERATED_LIST
    }
}

#[test]
fn mixed_implementation() {
    let static_provider = MixedImplementation{};
    assert!(static_provider.has_port("in"));
    assert!(static_provider.has_port("inout"));
    assert!(static_provider.has_port(INOUT));
    assert!(static_provider.has_port("out"));
    assert!(static_provider.has_port(OUT));
}

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
