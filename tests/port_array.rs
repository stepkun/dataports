// Copyright © 2026 Stephan Kunz
//! Test [`PortArray`]s public API.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use core::f64::consts::PI;

use dataports::{
	BoundInOutPort, BoundInPort, BoundOutPort, FlowingInOutPort, FlowingInPort, FlowingOutPort, Port, PortArray,
};

macro_rules! creation_bind {
	($value: expr) => {
		PortArray::new([
			Port::new("inbound", BoundInPort::new($value)),
			Port::new("outbound", BoundOutPort::new($value)),
			Port::new("inoutbound", BoundInOutPort::new($value)),
		])
	};
}

#[test]
fn bound() {
	let array = creation_bind!(true);
	let array = creation_bind!(42);
	let array = creation_bind!(PI);
	let array = creation_bind!("str");
	let array = creation_bind!(String::from("string"));
	let array = creation_bind!(vec![1, 2, 3]);
	let array = creation_bind!(vec!["1", "2", "3"]);
	let array = creation_bind!(vec![
		String::from("1"),
		String::from("2"),
		String::from("3")
	]);
	let array = creation_bind!(vec![vec![1, 2, 3], vec![4, 5, 6]]);
}

macro_rules! creation_flow {
	($value: expr) => {
		PortArray::new([
			Port::new("inflow", FlowingInPort::new($value)),
			Port::new("outflow", FlowingOutPort::new($value)),
			Port::new("inoutflow", FlowingInOutPort::new($value)),
		])
	};
}

#[test]
fn flowing() {
	let array = creation_flow!(42);
}
