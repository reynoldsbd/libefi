#![feature(lang_items)]
#![feature(panic_implementation)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]


#[macro_use]
extern crate efi;

use core::{
    panic::PanicInfo,
    ptr,
    slice,
};

use efi::{
    boot_services,
    boot_services::{
        AllocateType,
        Event,
        EventType,
        OpenProtocolAttributes,
        Protocol,
        SearchType,
        TPL,
    },
    protocols::{
        File,
        FileMode,
        FileAttributes,
        FileSystemInfo,
        SimpleFileSystem,
        SimpleTextInput,
    },
    SystemTable,
    types::{
        EfiRt,
        Handle,
        MemoryType,
        PhysicalAddress,
        Status,
    },
};


fn test_utf16_conversion(system_table: &SystemTable) -> Result<(), usize> {

    let mut num_errs = 0;
    efi_println!(system_table, "test UTF-16 conversion");

    efi_println!(system_table, "    test str to UTF-16");
    let src = "some string";
    match boot_services::str_to_utf16(src, &*(system_table.boot_services)) {
        Ok(buf) => {
            efi_println!(system_table, "#   buf: {:?}", buf);
        },
        Err(err) => {
            efi_println!(system_table, "!   failed to convert str to UTF-16");
            efi_println!(system_table, "!   {:?}", err);
            num_errs += 1;
        }
    }

    efi_println!(system_table, "    test UTF-16 to str");
    // Null-terminated "other string"
    let src: [u16; 13] = [0x6f,0x74,0x68,0x65,0x72,0x20,0x73,0x74,0x72,0x69,0x6e,0x67,0x00];
    match boot_services::utf16_to_str(&src, &*(system_table.boot_services)) {
        Ok(string) => {
            efi_println!(system_table, "#   string: {}", string);
        },
        Err(err) => {
            efi_println!(system_table, "!   failed to convert UTF-16 to str");
            efi_println!(system_table, "!   {:?}", err);
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
    efi_println!(system_table, "test events");

    efi_println!(system_table, "    test creating simple event");
    let simple_result = system_table.boot_services.create_event(
        EventType::empty(),
        TPL::Callback,
        empty_callback,
        &()
    );
    match simple_result {
        Ok(simple_event) => {

            efi_println!(system_table, "    test check new simple event");
            let simple_check_result = system_table.boot_services.check_event(simple_event);
            if let Ok(()) = simple_check_result {
                efi_println!(system_table, "!   simple event was already signaled");
                num_errs += 1;
            }

            efi_println!(system_table, "    test signal simple event");
            if let Err(err) = system_table.boot_services.signal_event(simple_event) {
                efi_println!(system_table, "!   failed to signal simple event");
                efi_println!(system_table, "!   {:?}", err);
                num_errs += 1;
            } else {

                efi_println!(system_table, "    test wait for simple event");
                let events = &[simple_event; 1];
                if let Err(err) = system_table.boot_services.wait_for_event(events) {
                    efi_println!(system_table, "!   failed to wait for simple event");
                    efi_println!(system_table, "!   {:?}", err);
                    num_errs += 1;
                }
            }

        },
        Err(err) => {
            efi_println!(system_table, "!   failed to create simple event");
            efi_println!(system_table, "!   {:?}", err);
            num_errs += 1;
        },
    }

    efi_println!(system_table, "    test creating event with callback");
    let simple_result = system_table.boot_services.create_event(
        EventType::NOTIFY_SIGNAL,
        TPL::Callback,
        echo_callback,
        &"callback message"
    );
    match simple_result {
        Ok(simple_event) => {

            efi_println!(system_table, "    test signal event with callback");
            if let Err(err) = system_table.boot_services.signal_event(simple_event) {
                efi_println!(system_table, "!   failed to signal event with callback");
                efi_println!(system_table, "!   {:?}", err);
                num_errs += 1;
            }

        },
        Err(err) => {
            efi_println!(system_table, "!   failed to create event with callback");
            efi_println!(system_table, "!   {:?}", err);
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
    efi_println!(system_table, "test errors");

    efi_println!(system_table, "    test page allocation");
    let mut addr: PhysicalAddress = ptr::null_mut();
    let res = system_table.boot_services.allocate_pages(
        AllocateType::AllocateAnyPages,
        MemoryType::LoaderData,
        1,
        &mut addr
    );
    if let Err(err) = res {
        efi_println!(system_table, "!   failed to allocate page");
        efi_println!(system_table, "!   {:?}", err);
        num_errs += 1;
    } else {
        efi_println!(system_table, "#   page allocated at {:p}", addr);

        efi_println!(system_table, "    test writing to allocated page");
        // Build a byte slice from the allocated memory, then attempt to write into that slice
        // There's no way to elegantly catch if this fails. Either the write will succeed, or the
        // system will catch due to an uncaught interrupt
        let mem = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 4096) };
        mem[0] = 1;

        efi_println!(system_table, "    test freeing page");
        if let Err(err) = system_table.boot_services.free_pages(addr, 1) {
            efi_println!(system_table, "!   failed to free page");
            efi_println!(system_table, "!   {:?}", err);
            num_errs += 1;
        }
    }

    efi_println!(system_table, "    test pool allocation");
    let res = system_table.boot_services.allocate_pool(MemoryType::LoaderData, 128);
    match res {
        Ok(buffer) => {
            efi_println!(system_table, "#   pool allocated at {:p}", buffer);

            efi_println!(system_table, "    test writing to allocated pool");
            // Build a byte slice from the allocated memory, then attempt to write into that slice
            // There's no way to elegantly catch if this fails. Either the write will succeed, or
            // the system will crash due to an uncaught interrupt
            let mem = unsafe { slice::from_raw_parts_mut(buffer, 128) };
            mem[0] = 1;

            efi_println!(system_table, "    test freeing pool");
            if let Err(err) = system_table.boot_services.free_pool(buffer) {
                efi_println!(system_table, "!   failed to free pool");
                efi_println!(system_table, "!   {:?}", err);
                num_errs += 1;
            }
        },
        Err(err) => {
            efi_println!(system_table, "!   failed to allocate pool");
            efi_println!(system_table, "!   {:?}", err);
            num_errs += 1;
        },
    }

    efi_println!(system_table, "    test memory map");
    match system_table.boot_services.get_memory_map() {
        Ok(map) => {
            efi_println!(system_table, "#   first entry: {:?}", map[0]);
        },
        Err(err) => {
            efi_println!(system_table, "!   failed to get memory map");
            efi_println!(system_table, "!   {:?}", err);
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
    efi_println!(system_table, "test protocols");

    efi_println!(system_table, "    test locate handle");
    let guid = SimpleTextInput::guid();
    match system_table.boot_services.locate_handle(SearchType::ByProtocol, Some(guid), None) {
        Ok(handles) => {
            efi_println!(system_table, "#   found {} handles for protocol", handles.len());

            efi_println!(system_table, "    test open protocol");
            let res = system_table.boot_services.open_protocol::<SimpleTextInput>(
                handles[0],
                image_handle,
                0,
                OpenProtocolAttributes::BY_HANDLE_PROTOCOL
            );
            match res {
                Ok(interface) => {
                    efi_println!(system_table, "    test close protocol");
                    let res = system_table.boot_services.close_protocol(
                        handles[0],
                        interface,
                        image_handle,
                        0
                    );
                    if let Err(err) = res {
                        efi_println!(system_table, "!   failed to close protocol");
                        efi_println!(system_table, "!   {:?}", err);
                        num_errs += 1;
                    }
                },
                Err(err) => {
                    efi_println!(system_table, "!   failed to open protocol");
                    efi_println!(system_table, "!   {:?}", err);
                    num_errs += 1;
                },
            }
        },
        Err(err) => {
            efi_println!(system_table, "!   failed to locate handle");
            efi_println!(system_table, "!   {:?}", err);
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
    efi_println!(system_table, "test files");

    efi_println!(system_table, "    test enumerating volumes");
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
                                        efi_println!(system_table, "#   volume label: {}", volume_label);

                                        if volume_label == "EFISys" {
                                            if let Err(err_count) = test_files_2(&root, system_table) {
                                                num_errs += err_count;
                                            }
                                        }
                                    },
                                    Err(err) => {
                                        efi_println!(system_table, "!   failed to get file system info");
                                        efi_println!(system_table, "!   {:?}", err);
                                        num_errs += 1;
                                    }
                                }
                            },
                            Err(err) => {
                                efi_println!(system_table, "!   failed to open volume");
                                efi_println!(system_table, "!   {:?}", err);
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
                            efi_println!(system_table, "!   failed to close file system protocol");
                            efi_println!(system_table, "!   {:?}", err);
                            num_errs += 1;
                        }
                    },
                    Err(err) => {
                        efi_println!(system_table, "!   failed to open file system protocol");
                        efi_println!(system_table, "!   {:?}", err);
                        num_errs += 1;
                        break;
                    },
                }
            }
        },
        Err(err) => {
            efi_println!(system_table, "!   failed to get volume handles");
            efi_println!(system_table, "!   {:?}", err);
            num_errs += 1;
        }
    }

    if num_errs > 0 {
        Err(num_errs)
    } else {
        Ok(())
    }
}


fn test_files_2(root: &File, system_table: &SystemTable) -> Result<(), usize> {

    let mut num_errs = 0;

    efi_println!(system_table, "    test open file");
    let path = boot_services::str_to_utf16("\\EFI\\test.txt", &system_table.boot_services)
        .unwrap();
    match root.open(&path, FileMode::READ, FileAttributes::empty()) {
        Ok(file) => {
            efi_println!(system_table, "    test read from file");
            let mut buf = [0u8; 20];
            match file.read(&mut buf) {
                Ok(len) => {
                    efi_println!(system_table, "#   read {} bytes", len);
                    efi_println!(system_table, "#   data: {:?}", &buf[0..len]);
                },
                Err(err) => {
                    efi_println!(system_table, "!   failed to read file");
                    efi_println!(system_table, "!   {:?}", err);
                    num_errs += 1;
                },
            }
        },
        Err(err) => {
            efi_println!(system_table, "!   failed to open file");
            efi_println!(system_table, "!   {:?}", err);
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

extern "win64" fn echo_callback(_: &Event, _: &&str) {}


#[no_mangle]
pub extern fn efi_main(image_handle: Handle, system_table: EfiRt<SystemTable>) -> Status {

    let mut total_errs = 0;

    unsafe {
        SYSTEM_TABLE = &*system_table;
    }

    if let Err(num_errs) = test_events(&system_table) {
        total_errs += num_errs;
    }

    if let Err(num_errs) = test_memory(&system_table) {
        total_errs += num_errs;
    }

    if let Err(num_errs) = test_protocols(image_handle, &system_table) {
        total_errs += num_errs;
    }

    if let Err(num_errs) = test_utf16_conversion(&system_table) {
        total_errs += num_errs;
    }

    if let Err(num_errs) = test_files(image_handle, &system_table) {
        total_errs += num_errs;
    }

    efi_println!(system_table, "tests completed with {} errors", total_errs);

    loop { }
}


/// Required Rust lang item
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() { }

static mut SYSTEM_TABLE: *const SystemTable = 0 as *const SystemTable;

/// Handles a panic by printing the error message to the screen
#[allow(private_no_mangle_fns)]
#[no_mangle]
#[panic_implementation]
fn panic_fmt(pi: &PanicInfo) -> ! {
    let sys_tab = unsafe { SYSTEM_TABLE.as_ref().unwrap() };

    efi_println!(sys_tab, "!!! PANIC !!!");
    if let Some(loc) = pi.location() {
        efi_println!(sys_tab, "Location: {}:{}:{}", loc.file(), loc.line(), loc.column());
    }
    if let Some(msg) = pi.message() {
        efi_println!(sys_tab, "{}", msg);
    }

    loop { }
}
