//! EFI services available at all times


use types::{
    Char16,
    Status,
    TableHeader,
};


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
