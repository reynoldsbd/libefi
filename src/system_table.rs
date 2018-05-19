//! Root EFI System Table

use {
    runtime_services::RuntimeServices,
    types::{
        EfiRt,
        TableHeader,
    },
};
#[cfg(feature = "boot-services")]
use {
    boot_services::BootServices,
    protocols::{
        SimpleTextInput,
        SimpleTextOutput,
    },
    types::Handle,
};


/// Contains pointers to the runtime and boot services tables
///
/// TODO:
///
/// * need some safe abstraction over configuration table, e.g. a struct that acts like an array?
#[cfg(feature = "boot-services")]
#[derive(Debug)]
#[repr(C)]
pub struct SystemTable {
    pub hdr: TableHeader,
    pub firmware_vendor: EfiRt<u16>,
    pub firmware_revision: u32,

    // Console protocol instances
    pub console_in_handle: Handle,
    pub con_in: EfiRt<SimpleTextInput>,
    pub console_out_handle: Handle,
    pub con_out: EfiRt<SimpleTextOutput>,
    pub standard_error_handle: Handle,
    pub stderr: EfiRt<SimpleTextOutput>,

    // Standard service tables
    pub runtime_services: EfiRt<RuntimeServices>,
    pub boot_services: EfiRt<BootServices>,

    // Configuration table
    pub number_of_table_entries: usize,
    pub configuration_table: usize, // TODO
}


/// Contains pointers to the runtime and boot services tables
///
/// TODO:
///
/// * need some safe abstraction over configuration table, e.g. a struct that acts like an array?
#[cfg(not(feature = "boot-services"))]
#[derive(Debug)]
#[repr(C)]
pub struct SystemTable {
    pub hdr: TableHeader,
    pub firmware_vendor: EfiRt<u16>,
    pub firmware_revision: u32,

    _reserved_0: usize,
    _reserved_1: usize,
    _reserved_2: usize,
    _reserved_3: usize,
    _reserved_4: usize,
    _reserved_5: usize,

    // Standard service tables
    pub runtime_services: EfiRt<RuntimeServices>,
    _reserved_6: usize,

    // Configuration table
    pub number_of_table_entries: usize,
    pub configuration_table: usize, // TODO
}
