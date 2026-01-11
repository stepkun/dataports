// Copyright © 2026 Stephan Kunz
//! Test [`PortList`]s public API.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use core::f64::consts::PI;

use dataports::{BoundInOutPort, BoundInPort, BoundOutPort, PortList, PortProvider, PortVariant};

macro_rules! test_creation {
	($tp:ty, $value: expr) => {{
		let mut list = PortList::new();
		assert!(
			list.add("inbound1", PortVariant::InBound(BoundInPort::with_value($value)))
				.is_ok()
		);
		assert!(
			list.add("outbound1", PortVariant::OutBound(BoundOutPort::with_value($value)))
				.is_ok()
		);
		assert!(
			list.add("inoutbound1", PortVariant::InOutBound(BoundInOutPort::with_value($value)))
				.is_ok()
		);
		assert!(
			list.add("inbound2", PortVariant::InBound(BoundInPort::with_value($value)))
				.is_ok()
		);
		assert!(
			list.add("outbound2", PortVariant::OutBound(BoundOutPort::with_value($value)))
				.is_ok()
		);
		assert!(
			list.add("inoutbound2", PortVariant::InOutBound(BoundInOutPort::with_value($value)))
				.is_ok()
		);

		assert!(list.find("inbound").is_none());
		assert!(list.find("inbound1").is_some());
		assert!(list.find("outbound2").is_some());

		assert!(list.find_mut("inoutbound").is_none());
		assert!(list.find_mut("outbound1").is_some());
		assert!(list.find_mut("inoutbound2").is_some());
	}};
}

#[test]
fn list_creation() {
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
