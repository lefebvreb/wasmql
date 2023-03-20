//! Js values are simply ids of values stored in a js array and owned by the host
//! js host environment.

use core::fmt::{self, Display};

use alloc::string::ToString;
use serde::{ser, de};

use super::ffi;