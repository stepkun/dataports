// Copyright © 2026 Stephan Kunz
//! Test [`PortList`]s public API.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use core::f64::consts::PI;

use dataports::{BoundInOutPort, BoundInPort, BoundOutPort, Port, PortList};

macro_rules! test_creation {
	($tp:ty, $value: expr) => {{
		let mut list = PortList::new();
		assert!(
			list.add(Port::new("inbound1", BoundInPort::with_value($value)))
				.is_ok()
		);
		assert!(
			list.add(Port::new("outbound1", BoundOutPort::with_value($value)))
				.is_ok()
		);
		assert!(
			list.add(Port::new("inoutbound1", BoundInOutPort::with_value($value)))
				.is_ok()
		);
		assert!(
			list.add(Port::new("inbound2", BoundInPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			list.add(Port::new("outbound2", BoundOutPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			list.add(Port::new("inoutbound2", BoundInOutPort::new::<$tp>()))
				.is_ok()
		);

		assert!(
			list.add(Port::new("inoutbound2", BoundInOutPort::new::<$tp>()))
				.is_err()
		);
		list
	}};
}

#[test]
fn list_creation() {
	let list = test_creation!(bool, true);
	let list = test_creation!(i32, 42);
	let list = test_creation!(f64, PI);
	let list = test_creation!(&str, "str");
	let list = test_creation!(String, String::from("string"));
	let list = test_creation!(Vec<i32>, vec![1, 2, 3]);
	let list = test_creation!(Vec<&str>, vec!["1", "2", "3"]);
	let list = test_creation!(
		Vec<String>,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3")
		]
	);
	let array = test_creation!(Vec<Vec<f64>>, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
}
