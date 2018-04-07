#![no_std]
#![no_main]


#[macro_use]
extern crate efi;

use efi::console;
use efi::protocols::ScanCode;
use efi::runtime::SYSTEM_TABLE;
use efi::types::{
    AllocateType,
    Color,
    Event,
    EventType,
    MemoryType,
    PhysicalAddress,
    Status,
    TPL,
};

use core::slice;


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
        EventType::NotifySignal,
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

    efi_println!("    test memory allocation");
    let mut addr: PhysicalAddress = 1;
    let res = SYSTEM_TABLE.boot_services.allocate_pages(
        AllocateType::AllocateAnyPages,
        MemoryType::EfiLoaderData,
        1,
        &mut addr
    );
    if let Err(err) = res {
        efi_println!("!   failed to allocate memory");
        efi_println!("!   {:?}", err);
        num_errs += 1;
    } else {
        efi_println!("#   page allocated at {:x}", addr);

        efi_println!("    test writing to allocated memory");
        // Build a byte slice from the allocated memory, then attempt to write into that slice
        // There's no way to elegantly catch if this fails. Either the write will succeed, or the
        // system will catch due to an uncaught interrupt
        let mem = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 4096) };
        mem[0] = 1;
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

    efi_println!("tests completed with {} errors", total_errs);

    loop { }
}
