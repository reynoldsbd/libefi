//! Runtime support for Rust EFI images
//!
//! This module contains the minimal runtime needed to host a Rust application in a UEFI
//! environment.


use {
    boot_services::{
        AllocateType,
        MemoryType,
    },
    SystemTable,
    types::{
        Handle,
        PhysicalAddress,
        Status,
    },
};

use alloc::{
    heap,
    heap::{
        AllocErr,
        Layout,
    },
};

use core::{
    fmt,
    ops,
};

use spin::RwLock;


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


/// Heap allocator for use in a pre-boot UEFI environment
///
/// TODO:
///
/// This implementation satisfies each request by allocating one or more entire pages. This makes
/// it relatively easy to honor alignment requests (each allocated block is always page-aligned).
/// However, this implementation is not ideal because it will lead to memory exhaustion very
/// quickly.
///
/// An alternative would be to use UEFI Boot Services' allocate_pool and free_pool functions. These
/// functions do not provide any way of controlling alignment, so sastisfying alignment requirements
/// with such an implementation would require over-allocating a large block and returning a properly
/// aligned pointer within that block that satisfies the requested Layout.
///
/// This strategy makes deallocation difficult, because the allocator would need to maintain a
/// mapping of allocations so that free_pool can be passed the true allocation addresses.
pub(crate) struct PageAllocator {}

unsafe impl<'a> heap::Alloc for &'a PageAllocator {

    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {

        if layout.align() > 4096 {
            return Err(AllocErr::Unsupported {
                details: "maximum supported alignment is 4096"
            });
        }

        let num_pages = 1 + layout.size() / 4096;
        let mut addr: PhysicalAddress = 1;
        let res = SYSTEM_TABLE.boot_services.allocate_pages(
            AllocateType::AllocateAnyPages,
            MemoryType::LoaderData,
            num_pages,
            &mut addr
        );

        if let Err(_) = res {
            Err(AllocErr::Exhausted {
                request: layout
            })
        } else {
            Ok(addr as *mut u8)
        }
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {

        let num_pages = 1 + layout.size() / 4096;
        SYSTEM_TABLE.boot_services.free_pages(ptr as PhysicalAddress, num_pages)
            .expect("failed to dealloc pages")
    }
}


/// Required Rust lang item
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}


/// Handles a panic by printing the error message to the screen
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(msg: fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {

    efi_println!("panic in file {}:{}:{}", file, line, col);
    efi_println!("{}", msg);
    loop {}
}
