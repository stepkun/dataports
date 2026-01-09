// Copyright © 2026 Stephan Kunz
//! Test [`Portmap`]s public API.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use core::f64::consts::PI;

use dataports::{BoundInOutPort, BoundInPort, BoundOutPort, Port, PortMap, PortProvider};

macro_rules! test_creation {
	($tp:ty, $value: expr) => {{
		let mut map = PortMap::new();
		assert!(
			map.add(Port::new("inbound1", BoundInPort::with_value($value)))
				.is_ok()
		);
		assert!(
			map.add(Port::new("outbound1", BoundOutPort::with_value($value)))
				.is_ok()
		);
		assert!(
			map.add(Port::new("inoutbound1", BoundInOutPort::with_value($value)))
				.is_ok()
		);
		assert!(
			map.add(Port::new("inbound2", BoundInPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			map.add(Port::new("outbound2", BoundOutPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			map.add(Port::new("inoutbound2", BoundInOutPort::new::<$tp>()))
				.is_ok()
		);

		assert!(
			map.add(Port::new("inoutbound2", BoundInOutPort::new::<$tp>()))
				.is_err()
		);

		assert!(map.find("inbound").is_none());
		assert!(map.find("inbound1").is_some());
		assert!(map.find("outbound2").is_some());

		assert!(map.find_mut("inoutbound").is_none());
		assert!(map.find_mut("outbound1").is_some());
		assert!(map.find_mut("inoutbound2").is_some());
	}};
}

#[test]
fn map_creation() {
	test_creation!(bool, true);
	test_creation!(i32, 42);
	test_creation!(f64, PI);
	test_creation!(&str, "str");
	test_creation!(String, String::from("string"));
	test_creation!(Vec<i32>, vec![1, 2, 3]);
	test_creation!(Vec<&str>, vec!["1", "2", "3"]);
	test_creation!(
		Vec<String>,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3")
		]
	);
	test_creation!(Vec<Vec<f64>>, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
}
