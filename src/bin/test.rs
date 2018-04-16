#![feature(lang_items)]
#![no_main]
#![no_std]


extern crate efi;

use core::{
    fmt,
    slice,
};

use efi::{
    boot_services,
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
    protocols::{
        FileSystemInfo,
        SimpleFileSystem,
        SimpleTextInput,
    },
    SystemTable,
    types::{
        Handle,
        PhysicalAddress,
        Status,
    },
};


/// Print text to the console
macro_rules! efi_print {
    ($con_out:expr, $($arg:tt)*) => ({
        use core::fmt::Write;
        (&*$con_out)
            .write_fmt(format_args!($($arg)*))
            .expect("could not write to console");
    });
}


/// Print a line of text to the console
macro_rules! efi_println {
    ($con_out:expr, $fmt:expr) => (efi_print!($con_out, concat!($fmt, "\r\n")));
    ($con_out:expr, $fmt:expr, $($arg:tt)*) => (efi_print!($con_out, concat!($fmt, "\r\n"), $($arg)*));
}


fn test_utf16_conversion(system_table: &SystemTable) -> Result<(), usize> {

    let mut num_errs = 0;
    efi_println!(system_table.con_out, "test UTF-16 conversion");

    efi_println!(system_table.con_out, "    test str to UTF-16");
    let src = "some string";
    match boot_services::str_to_utf16(src, &*(system_table.boot_services)) {
        Ok(buf) => {
            efi_println!(system_table.con_out, "#   buf: {:?}", buf);
        },
        Err(err) => {
            efi_println!(system_table.con_out, "!   failed to convert str to UTF-16");
            efi_println!(system_table.con_out, "!   {:?}", err);
            num_errs += 1;
        }
    }

    efi_println!(system_table.con_out, "    test UTF-16 to str");
    // Null-terminated "other string"
    let src: [u16; 13] = [0x6f,0x74,0x68,0x65,0x72,0x20,0x73,0x74,0x72,0x69,0x6e,0x67,0x00];
    match boot_services::utf16_to_str(&src, &*(system_table.boot_services)) {
        Ok(string) => {
            efi_println!(system_table.con_out, "#   string: {}", string);
        },
        Err(err) => {
            efi_println!(system_table.con_out, "!   failed to convert UTF-16 to str");
            efi_println!(system_table.con_out, "!   {:?}", err);
            num_errs += 1;
        }
    }

    if num_errs > 0 {
        Err(num_errs)
    } else {
        Ok(())
    }
}


fn test_events(system_table: &SystemTable) -> Result<(), usize> {

    let mut num_errs = 0;
    efi_println!(system_table.con_out, "test events");

    efi_println!(system_table.con_out, "    test creating simple event");
    let simple_result = system_table.boot_services.create_event(
        EventType::empty(),
        TPL::Callback,
        empty_callback,
        &()
    );
    match simple_result {
        Ok(simple_event) => {

            efi_println!(system_table.con_out, "    test check new simple event");
            let simple_check_result = system_table.boot_services.check_event(simple_event);
            if let Ok(()) = simple_check_result {
                efi_println!(system_table.con_out, "!   simple event was already signaled");
                num_errs += 1;
            }

            efi_println!(system_table.con_out, "    test signal simple event");
            if let Err(err) = system_table.boot_services.signal_event(simple_event) {
                efi_println!(system_table.con_out, "!   failed to signal simple event");
                efi_println!(system_table.con_out, "!   {:?}", err);
                num_errs += 1;
            } else {

                efi_println!(system_table.con_out, "    test wait for simple event");
                let events = &[simple_event; 1];
                if let Err(err) = system_table.boot_services.wait_for_event(events) {
                    efi_println!(system_table.con_out, "!   failed to wait for simple event");
                    efi_println!(system_table.con_out, "!   {:?}", err);
                    num_errs += 1;
                }
            }

        },
        Err(err) => {
            efi_println!(system_table.con_out, "!   failed to create simple event");
            efi_println!(system_table.con_out, "!   {:?}", err);
            num_errs += 1;
        },
    }

    efi_println!(system_table.con_out, "    test creating event with callback");
    let simple_result = system_table.boot_services.create_event(
        EventType::NOTIFY_SIGNAL,
        TPL::Callback,
        echo_callback,
        &"callback message"
    );
    match simple_result {
        Ok(simple_event) => {

            efi_println!(system_table.con_out, "    test signal event with callback");
            if let Err(err) = system_table.boot_services.signal_event(simple_event) {
                efi_println!(system_table.con_out, "!   failed to signal event with callback");
                efi_println!(system_table.con_out, "!   {:?}", err);
                num_errs += 1;
            }

        },
        Err(err) => {
            efi_println!(system_table.con_out, "!   failed to create event with callback");
            efi_println!(system_table.con_out, "!   {:?}", err);
            num_errs += 1;
        },
    }

    if num_errs > 0 {
        Err(num_errs)
    } else {
        Ok(())
    }
}


fn test_memory(system_table: &SystemTable) -> Result<(), usize> {

    let mut num_errs = 0;
    efi_println!(system_table.con_out, "test errors");

    efi_println!(system_table.con_out, "    test page allocation");
    let mut addr: PhysicalAddress = 1;
    let res = system_table.boot_services.allocate_pages(
        AllocateType::AllocateAnyPages,
        MemoryType::LoaderData,
        1,
        &mut addr
    );
    if let Err(err) = res {
        efi_println!(system_table.con_out, "!   failed to allocate page");
        efi_println!(system_table.con_out, "!   {:?}", err);
        num_errs += 1;
    } else {
        efi_println!(system_table.con_out, "#   page allocated at {:x}", addr);

        efi_println!(system_table.con_out, "    test writing to allocated page");
        // Build a byte slice from the allocated memory, then attempt to write into that slice
        // There's no way to elegantly catch if this fails. Either the write will succeed, or the
        // system will catch due to an uncaught interrupt
        let mem = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 4096) };
        mem[0] = 1;

        efi_println!(system_table.con_out, "    test freeing page");
        if let Err(err) = system_table.boot_services.free_pages(addr, 1) {
            efi_println!(system_table.con_out, "!   failed to free page");
            efi_println!(system_table.con_out, "!   {:?}", err);
            num_errs += 1;
        }
    }

    efi_println!(system_table.con_out, "    test pool allocation");
    let res = system_table.boot_services.allocate_pool(MemoryType::LoaderData, 128);
    match res {
        Ok(buffer) => {
            efi_println!(system_table.con_out, "#   pool allocated at {:p}", buffer);

            efi_println!(system_table.con_out, "    test writing to allocated pool");
            // Build a byte slice from the allocated memory, then attempt to write into that slice
            // There's no way to elegantly catch if this fails. Either the write will succeed, or
            // the system will crash due to an uncaught interrupt
            let mem = unsafe { slice::from_raw_parts_mut(buffer, 128) };
            mem[0] = 1;

            efi_println!(system_table.con_out, "    test freeing pool");
            if let Err(err) = system_table.boot_services.free_pool(buffer) {
                efi_println!(system_table.con_out, "!   failed to free pool");
                efi_println!(system_table.con_out, "!   {:?}", err);
                num_errs += 1;
            }
        },
        Err(err) => {
            efi_println!(system_table.con_out, "!   failed to allocate pool");
            efi_println!(system_table.con_out, "!   {:?}", err);
            num_errs += 1;
        },
    }

    efi_println!(system_table.con_out, "    test memory map");
    match system_table.boot_services.get_memory_map() {
        Ok(map) => {
            efi_println!(system_table.con_out, "#   first entry: {:?}", map[0]);
        },
        Err(err) => {
            efi_println!(system_table.con_out, "!   failed to get memory map");
            efi_println!(system_table.con_out, "!   {:?}", err);
            num_errs += 1;
        },
    }

    if num_errs > 0 {
        Err(num_errs)
    } else {
        Ok(())
    }
}


fn test_protocols(image_handle: Handle, system_table: &SystemTable) -> Result<(), usize> {

    let mut num_errs = 0;
    efi_println!(system_table.con_out, "test protocols");

    efi_println!(system_table.con_out, "    test locate handle");
    let guid = SimpleTextInput::guid();
    match system_table.boot_services.locate_handle(SearchType::ByProtocol, Some(guid), None) {
        Ok(handles) => {
            efi_println!(system_table.con_out, "#   found {} handles for protocol", handles.len());

            efi_println!(system_table.con_out, "    test open protocol");
            let res = system_table.boot_services.open_protocol::<SimpleTextInput>(
                handles[0],
                image_handle,
                0,
                OpenProtocolAttributes::BY_HANDLE_PROTOCOL
            );
            match res {
                Ok(interface) => {
                    efi_println!(system_table.con_out, "    test close protocol");
                    let res = system_table.boot_services.close_protocol(
                        handles[0],
                        interface,
                        image_handle,
                        0
                    );
                    if let Err(err) = res {
                        efi_println!(system_table.con_out, "!   failed to close protocol");
                        efi_println!(system_table.con_out, "!   {:?}", err);
                        num_errs += 1;
                    }
                },
                Err(err) => {
                    efi_println!(system_table.con_out, "!   failed to open protocol");
                    efi_println!(system_table.con_out, "!   {:?}", err);
                    num_errs += 1;
                },
            }
        },
        Err(err) => {
            efi_println!(system_table.con_out, "!   failed to locate handle");
            efi_println!(system_table.con_out, "!   {:?}", err);
            num_errs += 1;
        },
    }

    if num_errs > 0 {
        Err(num_errs)
    } else {
        Ok(())
    }
}


fn test_files(image_handle: Handle, system_table: &SystemTable) -> Result<(), usize> {

    let mut num_errs = 0;
    efi_println!(system_table.con_out, "test files");

    efi_println!(system_table.con_out, "    test enumerating volumes");
    let guid = SimpleFileSystem::guid();
    match system_table.boot_services.locate_handle(SearchType::ByProtocol, Some(guid), None) {
        Ok(handles) => {
            for handle in handles.iter() {
                let res = system_table.boot_services.open_protocol::<SimpleFileSystem>(
                    *handle,
                    image_handle,
                    0,
                    OpenProtocolAttributes::BY_HANDLE_PROTOCOL
                );
                match res {
                    Ok(file_system) => {
                        match file_system.open_volume() {
                            Ok(root) => {
                                let res = root.get_info::<FileSystemInfo>(
                                    &*(system_table.boot_services)
                                );
                                match res {
                                    Ok(fs_info) => {
                                        let volume_label = fs_info.volume_label(&*(system_table.boot_services))
                                            .unwrap();
                                        efi_println!(system_table.con_out, "#   volume label: {}", volume_label);
                                    },
                                    Err(err) => {
                                        efi_println!(system_table.con_out, "!   failed to get file system info");
                                        efi_println!(system_table.con_out, "!   {:?}", err);
                                        num_errs += 1;
                                    }
                                }
                            },
                            Err(err) => {
                                efi_println!(system_table.con_out, "!   failed to open volume");
                                efi_println!(system_table.con_out, "!   {:?}", err);
                                num_errs += 1;
                            },
                        }

                        let res = system_table.boot_services.close_protocol(
                            *handle,
                            file_system,
                            image_handle,
                            0
                        );
                        if let Err(err) = res {
                            efi_println!(system_table.con_out, "!   failed to close file system protocol");
                            efi_println!(system_table.con_out, "!   {:?}", err);
                            num_errs += 1;
                        }
                    },
                    Err(err) => {
                        efi_println!(system_table.con_out, "!   failed to open file system protocol");
                        efi_println!(system_table.con_out, "!   {:?}", err);
                        num_errs += 1;
                        break;
                    },
                }
            }
        },
        Err(err) => {
            efi_println!(system_table.con_out, "!   failed to get volume handles");
            efi_println!(system_table.con_out, "!   {:?}", err);
            num_errs += 1;
        }
    }

    if num_errs > 0 {
        Err(num_errs)
    } else {
        Ok(())
    }
}


extern "win64" fn empty_callback(_: &Event, _: &()) { }

extern "win64" fn echo_callback(_: &Event, _: &&str) {}


#[no_mangle]
pub extern fn efi_main(image_handle: Handle, system_table: &SystemTable) -> Status {

    let mut total_errs = 0;

    unsafe {
        SYSTEM_TABLE = system_table;
    }

    if let Err(num_errs) = test_events(system_table) {
        total_errs += num_errs;
    }

    if let Err(num_errs) = test_memory(system_table) {
        total_errs += num_errs;
    }

    if let Err(num_errs) = test_protocols(image_handle, system_table) {
        total_errs += num_errs;
    }

    if let Err(num_errs) = test_utf16_conversion(system_table) {
        total_errs += num_errs;
    }

    if let Err(num_errs) = test_files(image_handle, system_table) {
        total_errs += num_errs;
    }

    efi_println!(system_table.con_out, "tests completed with {} errors", total_errs);

    loop { }
}


/// Required Rust lang item
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

static mut SYSTEM_TABLE: *const SystemTable = 0 as *const SystemTable;

/// Handles a panic by printing the error message to the screen
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(msg: fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {

    unsafe {
        let system_table = SYSTEM_TABLE
            .as_ref()
            .unwrap();
        efi_println!(system_table.con_out, "panic in file {}:{}:{}", file, line, col);
        efi_println!(system_table.con_out, "{}", msg);
    }
    loop {}
}
