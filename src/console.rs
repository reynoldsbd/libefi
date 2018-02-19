//! High-level EFI console interface


use core::fmt;

use runtime::SYSTEM_TABLE;
use types::{
    Color,
    Status,
};


/// Clears the console
pub fn clear() -> Result<(), Status> {

    SYSTEM_TABLE.con_out.clear_screen()
}


/// Configures the console cursor's position and visibility
pub fn configure_cursor(column: usize, row: usize, visible: bool) -> Result<(), Status> {

    SYSTEM_TABLE.con_out.set_cursor_position(column, row)?;
    SYSTEM_TABLE.con_out.enable_cursor(visible)
}


/// Resets the UEFI console devices
///
/// Note that extended verification is not performed. This can be accomplished by calling the reset
/// functions directly on the input and output protocol structures.
pub fn reset() -> Result<(), Status> {

    SYSTEM_TABLE.con_in.reset(false)?;
    SYSTEM_TABLE.con_out.reset(false)
}


/// Sets the colors used by the EFI console
pub fn set_colors(foreground: Color, background: Color) -> Result<(), Status> {

    SYSTEM_TABLE.con_out.set_attribute(foreground, background)
}


/// Sets the output mode of the device
///
/// `mode_number` definitions can be retrieved by a call to `get_modes`
pub fn set_mode(mode_number: usize) -> Result<(), Status> {

    SYSTEM_TABLE.con_out.set_mode(mode_number)
}


/// Writes the given text to the console
pub fn write(s: &str) -> Result<(), Status> {

    SYSTEM_TABLE.con_out.output_string(s)
}


/// fmt::Writer for the EFI console
pub struct Writer {}

impl fmt::Write for Writer {

    #[allow(unused_must_use)]
    fn write_str(&mut self, s: &str) -> fmt::Result {

        // TODO: surface errors?
        write(s);
        Ok(())
    }
}


/// Print text to the console
#[macro_export]
macro_rules! efi_print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        ($crate::console::Writer { })
            .write_fmt(format_args!($($arg)*))
            .expect("could not write to console");
    });
}


/// Print a line of text to the console
#[macro_export]
macro_rules! efi_println {
    ($fmt:expr) => (efi_print!(concat!($fmt, "\r\n")));
    ($fmt:expr, $($arg:tt)*) => (efi_print!(concat!($fmt, "\r\n"), $($arg)*));
}
