//! Runtime support for Rust EFI images
//!
//! This module contains the minimal runtime needed to host a Rust application in a UEFI
//! environment.
//!
//! TODO:
//!
//! * ensure the correct lang_items are available and fully implemented (esp. panic_format)


use core::ops;

use spin::RwLock;

use types::{
    Handle,
    Status,
    SystemTable,
};


extern {
    /// Main EFI image entry point
    ///
    /// This function is expected to be supplied by the consumer of this crate. It should be tagged
    /// with the no_mangle attribute to ensure successful linkage
    ///
    /// TODO:
    ///
    /// * this raises an improper_ctypes warning. Investigate?
    fn efi_main() -> Status;
}


/// EFI entry point
///
/// This function is called immediately after an EFI image is loaded and relocated. It sets up some
/// internal static configuration and then calls into the main function.
#[no_mangle]
#[link_name = "_start"]
pub unsafe extern "win64" fn _start(_image_handle: Handle, system_table: &'static SystemTable)
    -> Status {

    SYSTEM_TABLE.set(system_table);
    efi_main()
}


/// TODO
pub struct SystemTableWrapper(RwLock<Option<&'static SystemTable>>);

impl SystemTableWrapper {

    const fn new() -> SystemTableWrapper {

        SystemTableWrapper(RwLock::new(None))
    }

    fn set(&self, system_table: &'static SystemTable) {

        *(self.0.write()) = Some(system_table);
    }
}

impl ops::Deref for SystemTableWrapper {
    type Target = SystemTable;

    fn deref(&self) -> &SystemTable {

        self.0.read()
            .expect("attempt to reference static runtime::SYSTEM_TABLE before initialization")
    }
}


/// Static reference to this image's EFI SystemTable
///
/// Because one focus of this library is to provide user friendly abstractions over the EFI API,
/// the runtime maintains an internal reference so it may call into the EFI system on behalf of the
/// user.
pub static SYSTEM_TABLE: SystemTableWrapper = SystemTableWrapper::new();


/// Required Rust lang item
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}


/// Handles a panic by printing the error message to the screen
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_format() {}
