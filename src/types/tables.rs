use core::ptr;
use core::sync::atomic::AtomicPtr;

use super::{
    AllocateType,
    Char16,
    Handle,
    MemoryType,
    OwnedPtr,
    PhysicalAddress,
    Status,
};

use protocols::{
    SimpleTextInput,
    SimpleTextOutput,
};


/// Data structure that precedes all of the standard EFI table types
#[repr(C)]
pub struct TableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}


/// Contains pointers to the runtime and boot services tables
///
/// TODO:
///
/// * need some safe abstraction over configuration table, e.g. a struct that acts like an array
#[repr(C)]
pub struct SystemTable {
    pub hdr: TableHeader,
    pub firmware_vendor: OwnedPtr<u16>,
    pub firmware_revision: u32,
    pub console_in_handle: Handle,
    pub con_in: OwnedPtr<SimpleTextInput>,
    pub console_out_handle: Handle,
    pub con_out: OwnedPtr<SimpleTextOutput>,
    pub standard_error_handle: Handle,
    pub stderr: OwnedPtr<SimpleTextOutput>,
    pub runtime_services: OwnedPtr<RuntimeServices>,
    pub boot_services: OwnedPtr<BootServices>,
    pub number_of_table_entries: usize,
    pub configuration_table: usize, // TODO
}


/// Contains pointers to all of the runtime services
#[repr(C)]
pub struct RuntimeServices {
    pub hdr: TableHeader,
    pub _get_time: extern "win64" fn(),
    pub _set_time: extern "win64" fn(),
    pub _get_wakeup_time: extern "win64" fn(),
    pub _set_wakeup_time: extern "win64" fn(),
    pub _set_virtual_address_map: extern "win64" fn(),
    pub _convert_pointer: extern "win64" fn(),
    pub _get_variable: extern "win64" fn(
        variable_name: *const Char16,
        vendor_guid: usize, // TODO
        attributes: &mut u32,
        data_size: &mut usize,
        data: *mut ()
    ) -> Status,
    pub _get_next_variable: extern "win64" fn(),
    pub _set_variable: extern "win64" fn(),
    pub _get_next_high_monotonic_count: extern "win64" fn(),
    pub _reset_system: extern "win64" fn(),
    pub _update_capsule: extern "win64" fn(),
    pub _query_capsule_capabilities: extern "win64" fn(),
    pub _query_variable_info: extern "win64" fn(),
}

impl RuntimeServices {

    pub fn get_time(&self) {
        unimplemented!();
    }

    pub fn set_time(&self) {
        unimplemented!();
    }

    pub fn get_wakeup_time(&self) {
        unimplemented!();
    }

    pub fn set_wakeup_time(&self) {
        unimplemented!();
    }

    pub fn set_virtual_address_map(&self) {
        unimplemented!();
    }

    pub fn convert_pointer(&self) {
        unimplemented!();
    }

    pub fn get_variable(&self) {
        unimplemented!();
    }

    pub fn get_next_variable(&self) {
        unimplemented!();
    }

    pub fn set_variable(&self) {
        unimplemented!();
    }

    pub fn get_next_high_monotonic_count(&self) {
        unimplemented!();
    }

    pub fn reset_system(&self) {
        unimplemented!();
    }

    pub fn update_capsule(&self) {
        unimplemented!();
    }

    pub fn query_capsule_capabilities(&self) {
        unimplemented!();
    }

    pub fn query_variable_info(&self) {
        unimplemented!();
    }
}


/// Contains pointers to all of the boot services
#[repr(C)]
pub struct BootServices {
    pub hdr: TableHeader,
    pub _raise_tpl: extern "win64" fn(),
    pub _restore_tpl: extern "win64" fn(),
    pub _allocate_pages: extern "win64" fn(
        allocation_type: AllocateType,
        memory_type: MemoryType,
        pages: usize,
        memory: PhysicalAddress
    ) -> Status,
    pub _free_pages: extern "win64" fn(
        memory: PhysicalAddress,
        pages: usize
    ) -> Status,
    pub _get_memory_map: extern "win64" fn(
        memory_map_size: &mut usize,
        memory_map: &mut usize, // TODO
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
    pub _create_event: extern "win64" fn(
        event_type: u32,
        notify_tpl: usize, // TODO
        notify_function: usize, // TODO
        notify_context: usize, // TODO
        event: &mut usize // TODO
    ) -> Status,
    pub _set_timer: extern "win64" fn(),
    pub _wait_for_event: extern "win64" fn(),
    pub _signal_event: extern "win64" fn(),
    pub _close_event: extern "win64" fn(),
    pub _check_event: extern "win64" fn(),
    pub _install_protocol_interface: extern "win64" fn(),
    pub _reinstall_protocol_interface: extern "win64" fn(),
    pub _uninstall_protocol_interface: extern "win64" fn(),
    pub _handle_protocol: extern "win64" fn(),
    reserved: AtomicPtr<()>,
    pub _register_protocol_notify: extern "win64" fn(),
    pub _locate_handle: extern "win64" fn(),
    pub _locate_device_path: extern "win64" fn(),
    pub _install_configuration_table: extern "win64" fn(),
    pub _load_image: extern "win64" fn(),
    pub _start_image: extern "win64" fn(),
    pub _exit: extern "win64" fn(),
    pub _unload_image: extern "win64" fn(),
    pub _exit_boot_services: extern "win64" fn(),
    pub _get_next_monotonic_count: extern "win64" fn(),
    pub _stall: extern "win64" fn(),
    pub _set_watchdog_timer: extern "win64" fn(),
    pub _connect_controller: extern "win64" fn(),
    pub _disconnect_controller: extern "win64" fn(),
    pub _open_protocol: extern "win64" fn(),
    pub _close_protocol: extern "win64" fn(),
    pub _open_protocol_information: extern "win64" fn(),
    pub _protocols_per_handle: extern "win64" fn(),
    pub _locate_handle_buffer: extern "win64" fn(),
    pub _locate_protocol: extern "win64" fn(),
    pub _install_multiple_protocol_interfaces: extern "win64" fn(),
    pub _uninstall_multiple_protocol_interfaces: extern "win64" fn(),
    pub _calculate_crc32: extern "win64" fn(),
    pub _copy_mem: extern "win64" fn(),
    pub _set_mem: extern "win64" fn(),
    pub _create_event_ex: extern "win64" fn(),
}

impl BootServices {

    /// Allocates memory pages from the system
    pub fn allocate_pages(&self,
                          allocation_type: AllocateType,
                          memory_type: MemoryType,
                          pages: usize,
                          memory: PhysicalAddress)
        -> Result<PhysicalAddress, Status> {

        (self._allocate_pages)(allocation_type, memory_type, pages, memory)
            .as_result()
            .map(|_| memory)
    }

    /// Frees memory pages
    pub fn free_pages(&self, memory: PhysicalAddress, pages: usize) -> Result<(), Status> {

        (self._free_pages)(memory, pages)
            .as_result()?;
        Ok(())
    }

    /// Returns a copy of the current memory map
    ///
    /// TODO:
    ///
    /// * create an efi::MemoryMap type that can be indexed to retrieve MemoryDescriptors
    pub fn get_memory_map(&self) {
        unimplemented!();
    }

    /// Allocates a memory region
    pub fn allocate_pool(&self, pool_type: MemoryType, size: usize) -> Result<*mut u8, Status> {

        let mut buffer: *mut u8 = ptr::null_mut();
        (self._allocate_pool)(pool_type, size, &mut buffer)
            .as_result()
            .map(|_| buffer)
    }

    /// Returns pool memory to the system
    pub fn free_pool(&self, buffer: *mut u8) -> Result<(), Status> {

        (self._free_pool)(buffer)
            .as_result()?;
        Ok(())
    }

    pub fn create_event(&self) {
        unimplemented!();
    }
}
