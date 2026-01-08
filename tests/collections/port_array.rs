// Copyright © 2026 Stephan Kunz
//! Test [`PortArray`]s public API.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use core::f64::consts::PI;

use dataports::{BoundInOutPort, BoundInPort, BoundOutPort, Port, PortArray};

macro_rules! test_creation {
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
	let array = test_creation!(true);
	let array = test_creation!(42);
	let array = test_creation!(PI);
	let array = test_creation!("str");
	let array = test_creation!(String::from("string"));
	let array = test_creation!(vec![1, 2, 3]);
	let array = test_creation!(vec!["1", "2", "3"]);
	let array = test_creation!(vec![
		String::from("1"),
		String::from("2"),
		String::from("3")
	]);
	let array = test_creation!(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
}
