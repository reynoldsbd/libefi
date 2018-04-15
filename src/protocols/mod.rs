//! EFI protocol definitions
//!
//! This module provides FFI-safe protocol definitions for the standard UEFI protocols.


mod console;
mod files;


pub use self::console::*;
pub use self::files::*;
