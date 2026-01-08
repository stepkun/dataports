// Copyright © 2026 Stephan Kunz
//! Test [`PortArray`]s public API.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use core::f64::consts::PI;

use dataports::{BoundInOutPort, BoundInPort, BoundOutPort, Port, PortArray};

macro_rules! test_creation {
	($tp:ty, $value: expr) => {
		PortArray::new([
			Port::new("inbound1", BoundInPort::with_value($value)),
			Port::new("outbound1", BoundOutPort::with_value($value)),
			Port::new("inoutbound1", BoundInOutPort::with_value($value)),
			Port::new("inbound2", BoundInPort::new::<$tp>()),
			Port::new("outbound2", BoundOutPort::new::<$tp>()),
			Port::new("inoutbound2", BoundInOutPort::new::<$tp>()),
		])
	};
}

#[test]
fn array_creation() {
	let array = test_creation!(bool, true);
	let array = test_creation!(i32, 42);
	let array = test_creation!(f64, PI);
	let array = test_creation!(&str, "str");
	let array = test_creation!(String, String::from("string"));
	let array = test_creation!(Vec<i32>, vec![1, 2, 3]);
	let array = test_creation!(Vec<&str>, vec!["1", "2", "3"]);
	let array = test_creation!(
		Vec<String>,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3")
		]
	);
	let array = test_creation!(Vec<Vec<f64>>, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
}
