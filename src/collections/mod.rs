// Copyright © 2026 Stephan Kunz
//! Module.

#![allow(unused)]

use crate::ConstString;

pub mod port_array;
mod port_list;
mod port_map;

/// Something that provides ports.
trait PortProvider {}
