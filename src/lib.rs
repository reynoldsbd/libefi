//! Crate for writing UEFI software using Rust


#![no_std]
#![feature(compiler_builtins_lib, const_fn, lang_items)]


extern crate compiler_builtins;
extern crate rlibc;
extern crate spin;


#[macro_use]
pub mod console;
pub mod protocols;
pub mod runtime;
pub mod types;
