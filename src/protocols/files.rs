use core::{
    mem,
    slice,
};

use {
    boot_services::{
        BootServices,
        Guid,
        MemoryType,
        Protocol,
        utf16_to_str,
    },
    types::{
        Bool,
        Char16,
        OwnedPtr,
        Status,
    },
};


/// Provides file based access to supported file systems
#[repr(C)]
pub struct File {
    pub revision: u64,
    pub _open: extern "win64" fn() -> Status,
    pub _close: extern "win64" fn(this: &File) -> Status,
    pub _delete: extern "win64" fn() -> Status,
    pub _read: extern "win64" fn() -> Status,
    pub _write: extern "win64" fn() -> Status,
    pub _get_position: extern "win64" fn() -> Status,
    pub _set_position: extern "win64" fn() -> Status,
    pub _get_info: extern "win64" fn(
        this: &File,
        information_type: &Guid,
        buffer_size: &mut usize,
        buffer: *mut u8
    ) -> Status,
    pub _set_info: extern "win64" fn() -> Status,
    pub _flush: extern "win64" fn() -> Status,
}

impl File {

    /// Closes this file
    pub fn close(&self) -> Result<(), Status> {

        (self._close)(self)
            .as_result()
            .map(|_| ())
    }

    /// Returns information about a file
    pub fn get_info<T>(&self, boot_services: &BootServices) -> Result<OwnedPtr<T>, Status>
        where T: FileInformationType + Sized {

        let mut buf_size = mem::size_of::<T>();
        let buf = boot_services.allocate_pool(MemoryType::LoaderData, buf_size)?;
        let res = (self._get_info)(self, T::guid(), &mut buf_size, buf);
        if res == Status::Success {
            // If the initial buffer happened to be large enough, return it
            // This should never happen, because the length of the file name or volume label should
            // always be greater than 1
            return Ok(unsafe { OwnedPtr::new_unchecked(buf as *mut T) });
        } else if res != Status::BufferTooSmall {
            return Err(res)
        }

        // Reallocate the buffer with the specified size
        boot_services.free_pool(buf)?;
        boot_services.allocate_pool(MemoryType::LoaderData, buf_size)?;
        (self._get_info)(self, T::guid(), &mut buf_size, buf)
            .as_result()
            .map(|_| unsafe { OwnedPtr::new_unchecked(buf as *mut T) })
    }
}


/// Type of information that can be retrieved about a file
pub trait FileInformationType {

    fn guid() -> &'static Guid;
}


/// Generic information about a file
#[derive(Debug)]
#[repr(C)]
pub struct FileInfo {
    pub size: u64,
    pub file_size: u64,
    pub physical_size: u64,
    pub create_time: usize, // TODO
    pub last_access_time: usize, // TODO
    pub modification_time: usize, // TODO
    pub attribute: u64, // TODO
    _file_name: Char16,
}

impl FileInformationType for FileInfo {

    fn guid() -> &'static Guid { &FILE_INFO_GUID }
}


/// Information about the system volume
#[derive(Debug)]
#[repr(C)]
pub struct FileSystemInfo {
    _size: usize,
    pub read_only: Bool,
    pub volume_size: u64,
    pub free_space: u64,
    pub block_size: u32,
    _volume_label: Char16,
}

impl FileSystemInfo {

    /// Gets the volume label
    ///
    /// The returned string is allocated in pool memory using the given BootServices, and should be
    /// freed with a call to free_pool
    pub fn volume_label(&self, boot_services: &BootServices) -> Result<OwnedPtr<str>, Status> {

        let buf = unsafe {
            let buf_size = self._size - (mem::size_of::<FileSystemInfo>() - mem::size_of::<Char16>());
            slice::from_raw_parts(&(self._volume_label), buf_size)
        };

        utf16_to_str(buf, boot_services)
    }
}

impl FileInformationType for FileSystemInfo {

    fn guid() -> &'static Guid { &FILE_SYSTEM_INFO_GUID }
}


static FILE_INFO_GUID: Guid = Guid {
    data_1: 0x09576e92,
    data_2: 0x6d3f,
    data_3: 0x11d2,
    data_4: [
        0x8e,
        0x39,
        0x00,
        0xa0,
        0xc9,
        0x69,
        0x72,
        0x3b,
    ],
};


static FILE_SYSTEM_INFO_GUID: Guid = Guid {
    data_1: 0x09576e93,
    data_2: 0x6d3f,
    data_3: 0x11d2,
    data_4: [
        0x8e,
        0x39,
        0x00,
        0xa0,
        0xc9,
        0x69,
        0x72,
        0x3b,
    ],
};


/// Provides a minimal interface for file-type access to a device
#[repr(C)]
pub struct SimpleFileSystem {
    pub revision: u64,
    pub _open_volume: extern "win64" fn(
        this: &SimpleFileSystem,
        root: &mut *mut File
    ) -> Status,
}

impl SimpleFileSystem {

    /// Opens the root directory on a volume
    pub fn open_volume(&self) -> Result<OwnedPtr<File>, Status> {

        let mut file = 0 as *mut File;
        (self._open_volume)(self, &mut file)
            .as_result()
            .map(|_| unsafe { OwnedPtr::new_unchecked(file) })
    }
}

impl Protocol for SimpleFileSystem {

    fn guid() -> &'static Guid { &SIMPLE_FILE_SYSTEM_GUID }
}


static SIMPLE_FILE_SYSTEM_GUID: Guid = Guid {
    data_1: 0x0964e5b22,
    data_2: 0x6459,
    data_3: 0x11d2,
    data_4: [
        0x8e,
        0x39,
        0x00,
        0xa0,
        0xc9,
        0x69,
        0x72,
        0x3b,
    ],
};