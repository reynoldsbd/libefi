#![feature(alloc)]
#![no_main]
#![no_std]


extern crate alloc;
#[macro_use]
extern crate efi;

use alloc::boxed::Box;

use core::slice;

use efi::{
    boot_services::{
        AllocateType,
        Event,
        EventType,
        MemoryType,
        OpenProtocolAttributes,
        Protocol,
        SearchType,
        TPL,
    },
    console,
    protocols::{
        SimpleTextInput,
        Color,
    },
    runtime::{
        IMAGE_HANDLE,
        SYSTEM_TABLE,
    },
    types::{
        PhysicalAddress,
        Status,
    },
};


fn test_console() -> Result<(), usize> {

    let mut num_errs = 0;
    efi_println!("test console");

    efi_println!("    test clearing console");
    if let Err(err) = console::clear() {
        efi_println!("!   failed to clear console");
        efi_println!("!   {:?}", err);
        num_errs += 1;
    }

    efi_println!("    test configuring cursor");
    if let Err(err) = console::configure_cursor(0, 1, true) {
        efi_println!("!    failed to confgure cursor");
        efi_println!("!   {:?}", err);
        num_errs += 1;
    }

    efi_println!("    test setting output mode");
    if let Err(err) = console::set_mode(2) {
        efi_println!("!   failed to set output mode");
        efi_println!("!   {:?}", err);
        num_errs += 1;
    }

    efi_println!("    test setting colors");
    if let Err(err) = console::set_colors(Color::LightGreen, Color::Black) {
        efi_println!("!   failed to set colors");
        efi_println!("!   {:?}", err);
        num_errs += 1;
    }

    efi_println!("    test writing string");
    if let Err(err) = console::write("#   foo\r\n") {
        efi_println!("!   failed to write string");
        efi_println!("!   {:?}", err);
        num_errs += 1;
    }

    efi_println!("    test read char");
    match console::read_char() {
        Ok(key) => {
            efi_println!("#   {:?}", key);
        },
        Err(err) => {
            efi_println!("!   failed to read char");
            efi_println!("!   {:?}", err);
            num_errs += 1;
        }
    }

    if num_errs > 0 {
        Err(num_errs)
    } else {
        Ok(())
    }
}


fn test_events() -> Result<(), usize> {

    let mut num_errs = 0;
    efi_println!("test events");

    efi_println!("    test creating simple event");
    let simple_result = SYSTEM_TABLE.boot_services.create_event(
        EventType::empty(),
        TPL::Callback,
        empty_callback,
        &()
    );
    match simple_result {
        Ok(simple_event) => {

            efi_println!("    test check new simple event");
            let simple_check_result = SYSTEM_TABLE.boot_services.check_event(simple_event);
            if let Ok(()) = simple_check_result {
                efi_println!("!   simple event was already signaled");
                num_errs += 1;
            }

            efi_println!("    test signal simple event");
            if let Err(err) = SYSTEM_TABLE.boot_services.signal_event(simple_event) {
                efi_println!("!   failed to signal simple event");
                efi_println!("!   {:?}", err);
                num_errs += 1;
            } else {

                efi_println!("    test wait for simple event");
                let events = &[simple_event; 1];
                if let Err(err) = SYSTEM_TABLE.boot_services.wait_for_event(events) {
                    efi_println!("!   failed to wait for simple event");
                    efi_println!("!   {:?}", err);
                    num_errs += 1;
                }
            }

        },
        Err(err) => {
            efi_println!("!   failed to create simple event");
            efi_println!("!   {:?}", err);
            num_errs += 1;
        },
    }

    efi_println!("    test creating event with callback");
    let simple_result = SYSTEM_TABLE.boot_services.create_event(
        EventType::NOTIFY_SIGNAL,
        TPL::Callback,
        echo_callback,
        &"callback message"
    );
    match simple_result {
        Ok(simple_event) => {

            efi_println!("    test signal event with callback");
            if let Err(err) = SYSTEM_TABLE.boot_services.signal_event(simple_event) {
                efi_println!("!   failed to signal event with callback");
                efi_println!("!   {:?}", err);
                num_errs += 1;
            }

        },
        Err(err) => {
            efi_println!("!   failed to create event with callback");
            efi_println!("!   {:?}", err);
            num_errs += 1;
        },
    }

    if num_errs > 0 {
        Err(num_errs)
    } else {
        Ok(())
    }
}


fn test_memory() -> Result<(), usize> {

    let mut num_errs = 0;
    efi_println!("test errors");

    efi_println!("    test page allocation");
    let mut addr: PhysicalAddress = 1;
    let res = SYSTEM_TABLE.boot_services.allocate_pages(
        AllocateType::AllocateAnyPages,
        MemoryType::LoaderData,
        1,
        &mut addr
    );
    if let Err(err) = res {
        efi_println!("!   failed to allocate page");
        efi_println!("!   {:?}", err);
        num_errs += 1;
    } else {
        efi_println!("#   page allocated at {:x}", addr);

        efi_println!("    test writing to allocated page");
        // Build a byte slice from the allocated memory, then attempt to write into that slice
        // There's no way to elegantly catch if this fails. Either the write will succeed, or the
        // system will catch due to an uncaught interrupt
        let mem = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 4096) };
        mem[0] = 1;

        efi_println!("    test freeing page");
        if let Err(err) = SYSTEM_TABLE.boot_services.free_pages(addr, 1) {
            efi_println!("!   failed to free page");
            efi_println!("!   {:?}", err);
            num_errs += 1;
        }
    }

    efi_println!("    test pool allocation");
    let res = SYSTEM_TABLE.boot_services.allocate_pool(MemoryType::LoaderData, 128);
    match res {
        Ok(buffer) => {
            efi_println!("#   pool allocated at {:p}", buffer);

            efi_println!("    test writing to allocated pool");
            // Build a byte slice from the allocated memory, then attempt to write into that slice
            // There's no way to elegantly catch if this fails. Either the write will succeed, or the
            // system will catch due to an uncaught interrupt
            let mem = unsafe { slice::from_raw_parts_mut(buffer, 128) };
            mem[0] = 1;

            efi_println!("    test freeing pool");
            if let Err(err) = SYSTEM_TABLE.boot_services.free_pool(buffer) {
                efi_println!("!   failed to free pool");
                efi_println!("!   {:?}", err);
                num_errs += 1;
            }
        },
        Err(err) => {
            efi_println!("!   failed to allocate pool");
            efi_println!("!   {:?}", err);
            num_errs += 1;
        },
    }

    efi_println!("    test memory map");
    match SYSTEM_TABLE.boot_services.get_memory_map() {
        Ok(map) => {
            efi_println!("#   memory map: {:?}", map);
            efi_println!("#   first entry: {:?}", map[0]);
        },
        Err(err) => {
            efi_println!("!   failed to get memory map");
            efi_println!("!   {:?}", err);
            num_errs += 1;
        },
    }

    efi_println!("    test global allocator");
    // Either the allocation will succeed or this thread will panic
    let heap_string = Box::new("hello from the heap");
    efi_println!("#   heap string: {}", heap_string);

    if num_errs > 0 {
        Err(num_errs)
    } else {
        Ok(())
    }
}


fn test_protocols() -> Result<(), usize> {

    let mut num_errs = 0;
    efi_println!("test protocols");

    efi_println!("    test locate handle");
    let guid = SimpleTextInput::guid();
    match SYSTEM_TABLE.boot_services.locate_handle(SearchType::ByProtocol, Some(guid), None) {
        Ok(handles) => {
            efi_println!("#   found {} handles for protocol", handles.len());

            efi_println!("    test open protocol");
            let res = SYSTEM_TABLE.boot_services.open_protocol::<SimpleTextInput>(
                handles[0],
                *IMAGE_HANDLE,
                0,
                OpenProtocolAttributes::BY_HANDLE_PROTOCOL
            );
            match res {
                Ok(interface) => {
                    efi_println!("    test close protocol");
                    let res = SYSTEM_TABLE.boot_services.close_protocol(
                        handles[0],
                        interface,
                        *IMAGE_HANDLE,
                        0
                    );
                    if let Err(err) = res {
                        efi_println!("!   failed to close protocol");
                        efi_println!("!   {:?}", err);
                        num_errs += 1;
                    }
                },
                Err(err) => {
                    efi_println!("!   failed to open protocol");
                    efi_println!("!   {:?}", err);
                    num_errs += 1;
                },
            }
        },
        Err(err) => {
            efi_println!("!   failed to locate handle");
            efi_println!("!   {:?}", err);
            num_errs += 1;
        },
    }

    if num_errs > 0 {
        Err(num_errs)
    } else {
        Ok(())
    }
}


extern "win64" fn empty_callback(_: &Event, _: &()) { }

extern "win64" fn echo_callback(_: &Event, message: &&str) {

    efi_println!("#   from callback: {}", message);
}


#[no_mangle]
pub extern fn efi_main() -> Status {

    let mut total_errs = 0;

    if let Err(num_errs) = test_console() {
        total_errs += num_errs;
    }

    if let Err(num_errs) = test_events() {
        total_errs += num_errs;
    }

    if let Err(num_errs) = test_memory() {
        total_errs += num_errs;
    }

    if let Err(num_errs) = test_protocols() {
        total_errs += num_errs;
    }

    efi_println!("tests completed with {} errors", total_errs);

    loop { }
}
