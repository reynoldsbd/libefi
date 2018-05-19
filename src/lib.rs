//! Crate for writing UEFI software using Rust


#![feature(const_fn, lang_items, ptr_internals)]
#![no_std]


#[cfg(feature = "boot-services")]
#[macro_use]
extern crate bitflags;
extern crate rlibc;


#[cfg(feature = "boot-services")]
pub mod boot_services;
#[cfg(feature = "boot-services")]
pub mod protocols;
pub mod runtime_services;
mod system_table;
pub mod types;

pub use system_table::SystemTable;


/// Print text to the console
#[cfg(feature = "boot-services")]
#[macro_export]
macro_rules! efi_print {
    ($system_table:expr, $($arg:tt)*) => ({
        use core::fmt::Write;
        (&*($system_table).con_out)
            .write_fmt(format_args!($($arg)*))
            .expect("could not write to console");
    });
}


/// Print a line of text to the console
#[cfg(feature = "boot-services")]
#[macro_export]
macro_rules! efi_println {
    ($system_table:expr, $fmt:expr) =>
        (efi_print!($system_table, concat!($fmt, "\r\n")));
    ($system_table:expr, $fmt:expr, $($arg:tt)*) =>
        (efi_print!($system_table, concat!($fmt, "\r\n"), $($arg)*));
}
