// Copyright © 2025 Stephan Kunz
//! [`dataports`](crate) macro implementations.

/// macro for creation of an input only port.
#[macro_export]
macro_rules! input_port {
    ($name:expr $(,)?) => {$crate::Port::In{name: $name}}
}

/// macro for creation of an input/output port.
#[macro_export]
macro_rules! inout_port {
    ($name:expr $(,)?) => {$crate::Port::InOut{name: $name}}
}

/// macro for creation of an output only port.
#[macro_export]
macro_rules! output_port {
    ($name:expr $(,)?) => {$crate::Port::Out{name: $name}}
}

/// macro for creation of a list of [`Port`](crate::Port)'s.
#[macro_export]
macro_rules! port_list {
	($($e:expr),* $(,)?) => {[$($e),*]};
}

/// macro for creation of a [`PortProvider`](crate::PortProvider) implementation.
#[macro_export]
macro_rules! port_provider {
	($name:ident, $($e:expr),* $(,)?) => {
        static LIST: [$crate::Port; $crate::_count_elements!($($e),*)] = [$($e),*];
        impl $crate::PortProvider for $name {
            fn portlist(&self) -> &[dataports::Port] {
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

