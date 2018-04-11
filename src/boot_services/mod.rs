//! EFI services available in a pre-boot environment

mod events;
mod memory;
mod protocols;

pub use self::events::*;
pub use self::memory::*;
pub use self::protocols::*;

use types::{
    Handle,
    PhysicalAddress,
    Status,
    TableHeader,
};

use core::sync::atomic::AtomicPtr;


/// Contains pointers to all of the boot services
#[repr(C)]
pub struct BootServices {
    pub hdr: TableHeader,

    // Task Priority Services
    pub _raise_tpl: extern "win64" fn(new_tpl: TPL) -> Status,
    pub _restore_tpl: extern "win64" fn(old_tpl: TPL) -> Status,

    // Memory Services
    pub _allocate_pages: extern "win64" fn(
        allocation_type: AllocateType,
        memory_type: MemoryType,
        pages: usize,
        memory: &mut PhysicalAddress
    ) -> Status,
    pub _free_pages: extern "win64" fn(
        memory: PhysicalAddress,
        pages: usize
    ) -> Status,
    pub _get_memory_map: extern "win64" fn(
        memory_map_size: &mut usize,
        memory_map: *mut MemoryDescriptor,
        map_key: &mut usize,
        descriptor_size: &mut usize,
        descriptor_version: &mut u32
    ) -> Status,
    pub _allocate_pool: extern "win64" fn(
        pool_type: MemoryType,
        size: usize,
        buffer: &mut *mut u8
    ) -> Status,
    pub _free_pool: extern "win64" fn(
        buffer: *mut u8,
    ) -> Status,

    // Event & Timer Services
    pub _create_event: extern "win64" fn(
        event_type: EventType,
        notify_tpl: TPL,
        notify_function: extern "win64" fn(event: &Event, context: *const ()),
        notify_context: *const (),
        event: &mut &Event
    ) -> Status,
    pub _set_timer: extern "win64" fn(
        event: &Event,
        timer_type: TimerDelay,
        trigger_time: u64
    ) -> Status,
    pub _wait_for_event: extern "win64" fn(
        number_of_events: usize,
        event: *const &Event,
        index: &mut usize
    ) -> Status,
    pub _signal_event: extern "win64" fn(event: &Event) -> Status,
    pub _close_event: extern "win64" fn(event: &Event) -> Status,
    pub _check_event: extern "win64" fn(event: &Event) -> Status,

    // Protocol Handler Services
    pub _install_protocol_interface: extern "win64" fn(),
    pub _reinstall_protocol_interface: extern "win64" fn(),
    pub _uninstall_protocol_interface: extern "win64" fn(),
    pub _handle_protocol: extern "win64" fn(),
    reserved: AtomicPtr<()>,
    pub _register_protocol_notify: extern "win64" fn(),
    pub _locate_handle: extern "win64" fn(
        search_type: SearchType,
        protocol: *const Guid,
        search_key: *const (),
        buffer_size: &mut usize,
        buffer: *mut Handle
    ) -> Status,
    pub _locate_device_path: extern "win64" fn(),
    pub _install_configuration_table: extern "win64" fn(),

    // Image Services
    pub _load_image: extern "win64" fn(),
    pub _start_image: extern "win64" fn(),
    pub _exit: extern "win64" fn(),
    pub _unload_image: extern "win64" fn(),
    pub _exit_boot_services: extern "win64" fn(),

    // Miscellaneous Services
    pub _get_next_monotonic_count: extern "win64" fn(),
    pub _stall: extern "win64" fn(),
    pub _set_watchdog_timer: extern "win64" fn(),

    // Driver Support Services
    pub _connect_controller: extern "win64" fn(),
    pub _disconnect_controller: extern "win64" fn(),

    // Open and Close Protocol Services
    pub _open_protocol: extern "win64" fn(
        handle: Handle,
        protocol: &Guid,
        interface: &mut *mut (),
        agent_handle: Handle,
        controller_handle: Handle,
        attributes: OpenProtocolAttributes
    ) -> Status,
    pub _close_protocol: extern "win64" fn(
        handle: Handle,
        protocol: &Guid,
        agent_handle: Handle,
        controller_handle: Handle
    ) -> Status,
    pub _open_protocol_information: extern "win64" fn(),

    // Library Services
    pub _protocols_per_handle: extern "win64" fn(),
    pub _locate_handle_buffer: extern "win64" fn(),
    pub _locate_protocol: extern "win64" fn(),
    pub _install_multiple_protocol_interfaces: extern "win64" fn(),
    pub _uninstall_multiple_protocol_interfaces: extern "win64" fn(),

    // 32-bit CRC Services
    pub _calculate_crc32: extern "win64" fn(),

    // Miscellaneous Services
    pub _copy_mem: extern "win64" fn(),
    pub _set_mem: extern "win64" fn(),
    pub _create_event_ex: extern "win64" fn(),
}
