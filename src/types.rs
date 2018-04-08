//! FFI-safe EFI types
//!
//! This module contains FFI-safe types that can be used to interact with a UEFI platform.


use core::convert;
use core::ops;
use core::ptr::NonNull;
use core::sync::atomic::AtomicPtr;


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


/// Opaque handle to some object
pub type Handle = AtomicPtr<()>;


/// Used to differentiate status codes
const HIGHBIT: usize = 0x8000_0000_0000_0000;


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
pub type PhysicalAddress = u64;


/// Status code
#[derive(Clone, Copy, Debug, PartialEq)]
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

/// Data structure that precedes all of the standard EFI table types
#[repr(C)]
pub struct TableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}


/// A virtual memory address
pub type VirtualAddress = u64;
