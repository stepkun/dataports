// Copyright © 2025 Stephan Kunz
//! [`dataports`](crate) macro implementations.

/// macro for creation of an input only port description.
#[macro_export]
macro_rules! input_port_description {
	($name:expr $(,)?) => {
		$crate::PortDescription::In {
			name: $name,
			description: None,
		}
	};

	($name:expr, $desc:expr $(,)?) => {
		$crate::PortDescription::In {
			name: $name,
			description: Some($desc),
		}
	};
}

/// macro for creation of an input/output port description.
#[macro_export]
macro_rules! inout_port_description {
	($name:expr $(,)?) => {
		$crate::PortDescription::InOut {
			name: $name,
			description: None,
		}
	};

	($name:expr, $desc:expr $(,)?) => {
		$crate::PortDescription::InOut {
			name: $name,
			description: Some($desc),
		}
	};
}

/// macro for creation of an output only port description.
#[macro_export]
macro_rules! output_port_description {
	($name:expr $(,)?) => {
		$crate::PortDescription::Out {
			name: $name,
			description: None,
		}
	};

	($name:expr, $desc:expr $(,)?) => {
		$crate::PortDescription::Out {
			name: $name,
			description: Some($desc),
		}
	};
}

/// macro for creation of a list of [`PortDescription`](crate::PortDescription)'s.
#[macro_export]
macro_rules! port_description_list {
	($($e:expr),* $(,)?) => {[$($e),*]};
}

/// macro for creation of a [`PortDescripptionProvider`](crate::PortDescriptionProvider) implementation.
#[macro_export]
macro_rules! port_description_provider {
	($name:ident, $($e:expr),* $(,)?) => {
        static LIST: [$crate::PortDescription; $crate::_count_elements!($($e),*)] = [$($e),*];
        impl $crate::PortDescriptionProvider for $name {
            fn port_description_list(&self) -> &[dataports::PortDescription] {
                &LIST
            }
        }
    };
}

/// Internal macro to count the number of arguments
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
