//! Root EFI System Table

use {
    boot_services::BootServices,
    protocols::{
        SimpleTextInput,
        SimpleTextOutput,
    },
    runtime_services::RuntimeServices,
    types::{
        Handle,
        OwnedPtr,
        TableHeader,
    },
};


/// Contains pointers to the runtime and boot services tables
///
/// TODO:
///
/// * need some safe abstraction over configuration table, e.g. a struct that acts like an array?
#[repr(C)]
pub struct SystemTable {
    pub hdr: TableHeader,
    pub firmware_vendor: OwnedPtr<u16>,
    pub firmware_revision: u32,

    // Console protocol instances
    pub console_in_handle: Handle,
    pub con_in: OwnedPtr<SimpleTextInput>,
    pub console_out_handle: Handle,
    pub con_out: OwnedPtr<SimpleTextOutput>,
    pub standard_error_handle: Handle,
    pub stderr: OwnedPtr<SimpleTextOutput>,

    // Standard service tables
    pub runtime_services: OwnedPtr<RuntimeServices>,
    pub boot_services: OwnedPtr<BootServices>,

    // Configuration table
    pub number_of_table_entries: usize,
    pub configuration_table: usize, // TODO
}
