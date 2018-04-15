use super::BootServices;

use {
    types::{
        PhysicalAddress,
        Status,
        VirtualAddress,
    },
};

use core::ops;


/// Type of memory allocation to perform
#[repr(C)]
pub enum AllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}


/// Describes a region of memory
#[derive(Debug)]
#[repr(C)]
pub struct MemoryDescriptor {
    pub memory_type: MemoryType,
    pub physical_start: PhysicalAddress,
    pub virtual_start: VirtualAddress,
    pub number_of_pages: u64,
    pub attribute: u64, // TODO: bitflags
}


/// Describes the system's current memory configuration
#[derive(Debug)]
pub struct MemoryMap {
    buffer: *mut MemoryDescriptor,
    descriptor_size: usize,
    descriptor_version: u32,
    pub key: usize,
    pub size: usize,
}

impl ops::Index<usize> for MemoryMap {
    type Output = MemoryDescriptor;

    fn index(&self, index: usize) -> &MemoryDescriptor {

        if index > self.size {
            panic!("MemoryMap index out of bounds");
        }

        // It would be convenient to use the offset method here, but descriptor_size may be variable
        unsafe {
            let addr = (self.buffer as usize) + (index * self.descriptor_size);
            (addr as *mut MemoryDescriptor).as_ref().unwrap()
        }
    }
}


/// Type of memory
#[derive(Debug)]
#[repr(u32)]
pub enum MemoryType {
    ReservedMemoryType,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    MaxMemoryType,
}


impl BootServices {

    /// Allocates memory pages from the system
    pub fn allocate_pages(&self,
                          allocation_type: AllocateType,
                          memory_type: MemoryType,
                          pages: usize,
                          memory: &mut PhysicalAddress)
        -> Result<(), Status> {

        (self._allocate_pages)(allocation_type, memory_type, pages, memory)
            .as_result()
            .map(|_| ())
    }

    /// Frees memory pages
    pub fn free_pages(&self, memory: PhysicalAddress, pages: usize) -> Result<(), Status> {

        (self._free_pages)(memory, pages)
            .as_result()
            .map(|_| ())
    }

    /// Returns the current memory map
    pub fn get_memory_map(&self) -> Result<MemoryMap, Status> {

        let mut map = MemoryMap {
            buffer: 0 as *mut MemoryDescriptor,
            descriptor_size: 0,
            descriptor_version: 0,
            key: 0,
            size: 0,
        };

        // Make a dummy call to _get_memory_map to get details about descriptor and map size
        let res = (self._get_memory_map)(
            &mut map.size,
            map.buffer,
            &mut map.key,
            &mut map.descriptor_size,
            &mut map.descriptor_version
        );
        if res != Status::BufferTooSmall {
            return Err(res);
        }

        // Get a suitably-sized buffer with a little breathing room
        map.size += map.descriptor_size * 3;
        map.buffer = self.allocate_pool(
            MemoryType::LoaderData,
            map.size
        )? as *mut MemoryDescriptor;

        // Make the true call to _get_memory_map with a real buffer
        (self._get_memory_map)(
            &mut map.size,
            map.buffer,
            &mut map.key,
            &mut map.descriptor_size,
            &mut map.descriptor_version
        )
            .as_result()?;

        // Fix up map.size and return
        map.size = map.size / map.descriptor_size;
        Ok(map)
    }

    /// Allocates pool memory
    pub fn allocate_pool(&self, pool_type: MemoryType, size: usize) -> Result<*mut u8, Status> {

        let mut buffer: *mut u8 = 0 as *mut u8;
        (self._allocate_pool)(pool_type, size, &mut buffer)
            .as_result()
            .map(|_| buffer)
    }

    /// Returns pool memory to the system
    pub fn free_pool(&self, buffer: *mut u8) -> Result<(), Status> {

        (self._free_pool)(buffer)
            .as_result()
            .map(|_| ())
    }
}
