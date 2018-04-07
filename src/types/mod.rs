//! FFI-safe EFI types
//!
//! This module contains FFI-safe types that can be used to interact with a UEFI platform.


use core::convert;
use core::ops;
use core::ptr::NonNull;
use core::sync::atomic::AtomicPtr;


mod tables;

pub use self::tables::*;


/// Type of memory allocation to perform
///
/// TODO:
///
/// * UEFI does not specify concrete values for this enum, just a C `typedef enum`. All we can do
///   here is `repr(C)` and hope for the best
#[repr(C)]
pub enum AllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}


/// Logical boolean
#[repr(u8)]
pub enum Bool {
    False = 0,
    True = 1,
}

impl convert::From<bool> for Bool {

    fn from(b: bool) -> Self {

        match b {
            false => Bool::False,
            true => Bool::True,
        }
    }
}


/// 2-byte character
pub type Char16 = u16;


/// Colors supported by the UEFI console
#[derive(Clone, Copy, Debug)]
#[repr(usize)]
pub enum Color {
    Black = 0x00,
    Blue = 0x01,
    Green = 0x02,
    Cyan = 0x03,
    Red = 0x04,
    Magenta = 0x05,
    Brown = 0x06,
    LightGray = 0x07,
    DarkGray = 0x08,
    LightBlue = 0x09,
    LightGreen = 0x0a,
    LightCyan = 0x0b,
    LightRed = 0x0c,
    LightMagenta = 0x0d,
    Yellow = 0x0e,
    White = 0x0f,
}

impl Color {

    /// Tells whether this is a legal background color
    ///
    /// According to the UEFI spec, only certain colors may be legally used for the console's
    /// background.
    pub fn is_background(&self) -> bool {

        match *self as usize {
            0x00...0x07 => true,
            _ => false,
        }
    }
}


/// UEFI Event
#[derive(Debug)]
pub struct Event(pub(in types) ());

unsafe impl Sync for Event { }


bitflags! {
    /// Specifies an Event's mode and attributes
    pub struct EventType: u32 {
        const Time = 0x8000_0000;
        const Runtime = 0x4000_0000;
        const NotifyWait = 0x0000_0100;
        const NotifySignal = 0x0000_0200;
        const SignalExitBootServices = 0x0000_0201;
        const SignalVirtualAddressChange = 0x6000_0202;
    }
}


/// Opaque handle to some object
pub type Handle = AtomicPtr<()>;


/// Used to differentiate status codes
const HIGHBIT: usize = 0x8000_0000_0000_0000;


/// Describes a region of memory
#[repr(C)]
pub struct MemoryDescriptor {
    pub memory_type: MemoryType,
    pub physical_start: PhysicalAddress,
    pub virtual_start: VirtualAddress,
    pub number_of_pages: u64,
    pub attribute: u64, // TODO: bitflags
}


/// Type of memory
///
/// TODO:
///
/// * UEFI does not specify concrete values for this enum, just a C `typedef enum` (and the use of
///   u32 in function signatures). All we can do here is `repr(u32)` and hope for the best
#[repr(u32)]
pub enum MemoryType {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiMaxMemoryType,
}


/// Pointer to an owned item
///
/// This is a custom smart-pointer type used to provide a safe interface over the pointers used in
/// EFI tables. It allows only immutable dereferencing and panics if the underlying pointer is set
/// to null.
#[repr(C)]
pub struct OwnedPtr<T: ?Sized>(Option<NonNull<T>>);

impl<T: ?Sized> ops::Deref for OwnedPtr<T> {
    type Target = T;

    fn deref(&self) -> &T {

        if let OwnedPtr(Some(ref ptr)) = *self {
            unsafe { return ptr.as_ref(); }
        } else {
            panic!("attempt to dereference null owned ptr");
        }
    }
}

// NonNull is not Sync because the underlying data may be aliased. Assuming OwnedPtr is only used in
// FFI type definitions and never actually instantiated, we can be reasonably sure there is no
// aiasing, at least within the current application.
unsafe impl<T: ?Sized + Sync> Sync for OwnedPtr<T> { }


/// A physical memory address
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PhysicalAddress(u64);


/// Status code
#[derive(Clone, Copy, Debug)]
#[repr(usize)]
pub enum Status {
    Success = 0,
    LoadError = HIGHBIT | 1,
    InvalidParameter = HIGHBIT | 2,
    Unsupported = HIGHBIT | 3,
    BadBufferSize = HIGHBIT | 4,
    BufferTooSmall = HIGHBIT | 5,
    NotReady = HIGHBIT | 6,
    DeviceError = HIGHBIT | 7,
    WriteProtected = HIGHBIT | 8,
    OutOfResources = HIGHBIT | 9,
    VolumeCorrupted = HIGHBIT | 10,
    VolumeFull = HIGHBIT | 11,
    NoMedia = HIGHBIT | 12,
    MediaChanged = HIGHBIT | 13,
    NotFound = HIGHBIT | 14,
    AccessDenied = HIGHBIT | 15,
    NoResponse = HIGHBIT | 16,
    NoMapping = HIGHBIT | 17,
    Timeout = HIGHBIT | 18,
    NotStarted = HIGHBIT | 19,
    AlreadyStarted = HIGHBIT | 20,
    Aborted = HIGHBIT | 21,
    IcmpError = HIGHBIT | 22,
    TftpError = HIGHBIT | 23,
    ProtocolError = HIGHBIT | 24,
    IncompatibleVersion = HIGHBIT | 25,
    SecurityViolation = HIGHBIT | 26,
    CrcError = HIGHBIT | 27,
    EndOfMedia = HIGHBIT | 28,
    EndOfFile = HIGHBIT | 31,
    InvalidLanguage = HIGHBIT | 32,
    CompromisedData = HIGHBIT | 33,
    IpAddressConflict = HIGHBIT | 34,
    HttpError = HIGHBIT | 35,
    WarnUnknownGlyph = 1,
    WarnDeleteFailure = 2,
    WarnWriteFailure = 3,
    WarnBufferTooSmall = 4,
    WarnStaleData = 5,
    WarnFileSystem = 6,
    WarnResetRequired = 7,
}

impl Status {

    /// Converts the status to a Result type
    ///
    /// According to the EFI specification, negative status codes are considered errors, and zero or
    /// above is considered success. However, even a successful status code might have include a
    /// useful warning, so it is preserved here in the Result's Ok variant.
    ///
    /// TODO:
    ///
    /// * as an alternative, this could be done with core::ops::Try
    pub fn as_result(&self) -> Result<Status, Status> {

        // If HIGHBIT is set, this is an error
        if (*self as usize) & HIGHBIT != 0 {
            Err(*self)
        } else {
            Ok(*self)
        }
    }
}


/// Defines the type of a timer
#[repr(C)]
pub enum TimerDelay {
    Cancel,
    Periodic,
    Relative,
}


/// UEFI Task Priority Level
#[derive(Clone, Copy, Debug)]
#[repr(usize)]
pub enum TPL {
    Application = 4,
    Callback = 8,
    Notify = 16,
    HightLevel = 31,
}


/// A virtual memory address
#[repr(C)]
pub struct VirtualAddress(u64);
