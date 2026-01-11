// Copyright © 2026 Stephan Kunz
//! Test [`PortArray`]s public API.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use core::f64::consts::PI;

use dataports::{BoundInOutPort, BoundInPort, BoundOutPort, PortAccessors, PortArray, PortProvider, PortVariant};

macro_rules! test_creation {
	($tp:ty, $value: expr) => {
		let mut array = PortArray::new([
			("inbound0".into(), PortVariant::InBound(BoundInPort::new::<$tp>())),
			("outbound0".into(), PortVariant::OutBound(BoundOutPort::new::<$tp>())),
			(
				"inoutbound0".into(),
				PortVariant::InOutBound(BoundInOutPort::new::<$tp>()),
			),
			("inbound1".into(), PortVariant::InBound(BoundInPort::with_value($value))),
			(
				"outbound1".into(),
				PortVariant::OutBound(BoundOutPort::with_value($value)),
			),
			(
				"inoutbound1".into(),
				PortVariant::InOutBound(BoundInOutPort::with_value($value)),
			),
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
		assert!(array.find("outbound").is_none());
		assert!(array.find_mut("inoutbound").is_none());

		assert!(array.find("inbound0").is_some());
		assert!(array.find("inbound1").is_some());
		assert!(array.find("inbound2").is_some());

		assert!(array.find("outbound0").is_some());
		assert!(array.find("outbound1").is_some());
		assert!(array.find("outbound2").is_some());

		assert!(array.find_mut("inoutbound0").is_some());
		assert!(array.find_mut("inoutbound1").is_some());
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

macro_rules! test_accessors {
	($tp:ty, $value1: expr, $value2: expr) => {
		let mut array = PortArray::new([
			("inbound0".into(), PortVariant::InBound(BoundInPort::new::<$tp>())),
			("outbound0".into(), PortVariant::OutBound(BoundOutPort::new::<$tp>())),
			(
				"inoutbound0".into(),
				PortVariant::InOutBound(BoundInOutPort::new::<$tp>()),
			),
			("inbound1".into(), PortVariant::create_inbound($value1)),
			("outbound1".into(), PortVariant::create_outbound($value1)),
			("inoutbound1".into(), PortVariant::create_inoutbound($value1)),
		]);

		assert!(array.get::<$tp>("test").is_err());
		assert_eq!(array.get::<$tp>("inbound0").unwrap(), None);
		assert!(array.get::<$tp>("outbound0").is_err());
		assert_eq!(array.get::<$tp>("inoutbound0").unwrap(), None);
		assert_eq!(array.get::<$tp>("inbound1").unwrap(), Some($value1));
		assert!(array.get::<$tp>("outbound1").is_err());
		assert_eq!(array.get::<$tp>("inoutbound1").unwrap(), Some($value1));

		assert!(array.read::<$tp>("test").is_err());
		assert!(array.read::<$tp>("inbound0").is_err());
		assert!(array.read::<$tp>("inoutbound0").is_err());
		assert!(array.read::<$tp>("outbound0").is_err());
		assert_eq!(*array.read::<$tp>("inbound1").unwrap(), $value1);
		assert!(array.read::<$tp>("outbound1").is_err());
		assert_eq!(*array.read::<$tp>("inoutbound1").unwrap(), $value1);

		assert!(array.try_read::<$tp>("test").is_err());
		assert!(array.try_read::<$tp>("inbound0").is_err());
		assert!(array.try_read::<$tp>("inoutbound0").is_err());
		assert!(array.try_read::<$tp>("outbound0").is_err());
		assert_eq!(*array.try_read::<$tp>("inbound1").unwrap(), $value1);
		assert!(array.try_read::<$tp>("outbound1").is_err());
		assert_eq!(*array.try_read::<$tp>("inoutbound1").unwrap(), $value1);

		assert!(array.set("test", $value2).is_err());
		assert!(array.set("inbound0", $value2).is_err());
		assert!(array.set("outbound0", $value2).is_ok());
		//@TODO:!! assert!(array.set("inoutbound0", $value2).is_ok());
		assert!(array.set("inbound1", $value2).is_err());
		assert!(array.set("outbound1", $value2).is_ok());
		assert!(array.set("inoutbound1", $value2).is_ok());
	};
}

#[test]
fn array_accessors() {
	test_accessors!(bool, true, false);
}

#[test]
fn array_connection() {}
