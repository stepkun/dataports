// Copyright © 2026 Stephan Kunz
//! Test [`PortArray`]s public API.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use core::f64::consts::PI;

use dataports::{BoundInOutPort, BoundInPort, BoundOutPort, PortArray, PortProvider, PortVariant};

macro_rules! test_creation {
	($tp:ty, $value: expr) => {
		let mut array = PortArray::new([
			("inbound1".into(), PortVariant::InBound(BoundInPort::with_value($value))),
			(
				"outbound1".into(),
				PortVariant::OutBound(BoundOutPort::with_value($value)),
			),
			(
				"inoutbound1".into(),
				PortVariant::InOutBound(BoundInOutPort::with_value($value)),
			),
			("inbound2".into(), PortVariant::create_inbound($value)),
			("outbound2".into(), PortVariant::create_outbound($value)),
			("inoutbound2".into(), PortVariant::create_inoutbound($value)),
		]);

		assert!(array.find("inbound").is_none());
		assert!(array.find("inbound1").is_some());
		assert!(array.find("outbound2").is_some());

		assert!(array.find_mut("inoutbound").is_none());
		assert!(array.find_mut("outbound1").is_some());
		assert!(array.find_mut("inoutbound2").is_some());
	};
}

#[test]
fn array_creation() {
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
