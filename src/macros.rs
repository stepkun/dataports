// Copyright © 2025 Stephan Kunz
//! [`dataports`](crate) macro implementations.

/// Macro for creation of an input only [`PortDescription`](crate::PortDescription).
#[macro_export]
macro_rules! input_port_description {
	($tp:ty, $name:expr $(,)?) => {
		$crate::PortDescription::In($crate::port_description::PortDescriptionData::new(
			stringify!($tp),
			$name,
			None,
			None,
		))
	};

	($tp:ty, $name:expr, $default:expr $(,)?) => {
		$crate::PortDescription::In($crate::port_description::PortDescriptionData::new(
			stringify!($tp),
			$name,
			None,
			None,
		))
	};

	($tp:ty, $name:expr, $default:expr, $comment:expr $(,)?) => {
		$crate::PortDescription::In($crate::port_description::PortDescriptionData::new(
			stringify!($tp),
			$name,
			None,
			Some($comment),
		))
	};
}

/// Macro for creation of an input/output [`PortDescription`](crate::PortDescription).
#[macro_export]
macro_rules! inout_port_description {
	($tp:ty, $name:expr $(,)?) => {
		$crate::PortDescription::InOut($crate::port_description::PortDescriptionData::new(
			stringify!($tp),
			$name,
			None,
			None,
		))
	};

	($tp:ty, $name:expr, $default:expr $(,)?) => {
		$crate::PortDescription::InOut($crate::port_description::PortDescriptionData::new(
			stringify!($tp),
			$name,
			None,
			None,
		))
	};

	($tp:ty, $name:expr, $default:expr, $comment:expr $(,)?) => {
		$crate::PortDescription::InOut($crate::port_description::PortDescriptionData::new(
			stringify!($tp),
			$name,
			None,
			Some($comment),
		))
	};
}

/// Macro for creation of an output only [`PortDescription`](crate::PortDescription).
#[macro_export]
macro_rules! output_port_description {
	($tp:ty, $name:expr $(,)?) => {
		$crate::PortDescription::Out($crate::port_description::PortDescriptionData::new(
			stringify!($tp),
			$name,
			None,
			None,
		))
	};

	($tp:ty, $name:expr, $default:expr $(,)?) => {
		$crate::PortDescription::Out($crate::port_description::PortDescriptionData::new(
			stringify!($tp),
			$name,
			None,
			None,
		))
	};

	($tp:ty, $name:expr, $default:expr, $comment:expr $(,)?) => {
		$crate::PortDescription::Out($crate::port_description::PortDescriptionData::new(
			stringify!($tp),
			$name,
			None,
			Some($comment),
		))
	};
}

/// Macro for creation of a list of [`PortDescription`](crate::PortDescription)'s.
#[macro_export]
macro_rules! port_description_list {
	($($e:expr),* $(,)?) => {[$($e),*]};
}

/// Macro for creation of a [`PortDescripptionProvider`](crate::PortDescriptionProvider) implementation.
#[macro_export]
macro_rules! port_description_provider {
	// creation of an empty list
	($name:ident $(,)?) => {
        static LIST: [$crate::PortDescription; 0] = [];
        impl $crate::PortDescriptionProvider for $name {
            fn port_description_list(&self) -> &[dataports::PortDescription] {
                &LIST
            }
        }
    };

	// creation of a non-empty list
	($name:ident, $($e:expr),* $(,)?) => {
        static LIST: [$crate::PortDescription; $crate::_count_elements!($($e),*)] = [$($e),*];
        impl $crate::PortDescriptionProvider for $name {
            fn port_description_list(&self) -> &[dataports::PortDescription] {
                &LIST
            }
        }
    };
}

/// Internal macro to count the number of arguments.
///
/// The macro is expanded to an expression like `1+1+1+1+1`,
/// which is evaluated at compile time, so no runtime consts.
#[macro_export]
macro_rules! _count_elements {
    // Base case: if there are no arguments, count is 0
    () => {
        0
    };

    // End case if there is one argument left
    ($last: expr) => {
        1
    };

    // Recursive case: count one more argument
    ($first:expr, $($rest:expr),*) => {
        1 + $crate::_count_elements!($($rest),*)
    };
}
