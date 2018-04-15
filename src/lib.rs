//! Crate for writing UEFI software using Rust


#![feature(const_fn, lang_items, ptr_internals)]
#![no_std]


#[macro_use]
extern crate bitflags;
extern crate rlibc;


pub mod boot_services;
pub mod protocols;
pub mod runtime_services;
mod system_table;
pub mod types;

pub use system_table::SystemTable;
