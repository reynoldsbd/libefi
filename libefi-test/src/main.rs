#![no_std]
#![no_main]


#[macro_use]
extern crate efi;

use efi::types::Status;


#[no_mangle]
pub extern fn efi_main() -> Status {

    efi_println!("hello, world!");
    efi_println!("one: {}", 1);

    loop { }
}
