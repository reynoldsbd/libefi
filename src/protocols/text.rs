use types::{
    Bool,
    Char16,
    Color,
    OwnedPtr,
    Status,
};


/// This protocol is used to obtain input from the ConsoleIn device
///
/// TODO:
///
/// * implement EFI events so that wait_for_key can be used
#[repr(C)]
pub struct SimpleTextInput {
    pub _reset: extern "win64" fn(this: &SimpleTextInput, extended_verification: Bool) -> Status,
    pub _read_key_stroke: extern "win64" fn(this: &SimpleTextInput, key: &mut InputKey) -> Status,
    pub wait_for_key: usize, // TODO: Event
}

impl SimpleTextInput {

    /// Reset the ConsoleIn device
    pub fn reset(&self, extended_verification: bool) -> Result<(), Status> {

        (self._reset)(self, Bool::from(extended_verification))
            .as_result()?;
        Ok(())
    }

    /// Returns the next input character
    pub fn read_key_stroke(&self) -> Result<InputKey, Status> {

        let mut key = InputKey::default();
        (self._read_key_stroke)(self, &mut key)
            .as_result()
            .map(|_| key)
    }
}


/// Describes a keystroke
///
/// TODO:
///
/// * use an enum for scan_code
#[derive(Clone, Copy, Debug, Default)]
pub struct InputKey {
    pub scan_code: u16,
    pub unicode_char: Char16,
}


/// This protocol is used to control text-based output devices
#[repr(C)]
pub struct SimpleTextOutput {
    pub _reset: extern "win64" fn(this: &SimpleTextOutput, extended_verification: Bool) -> Status,
    pub _output_string: extern "win64" fn(this: &SimpleTextOutput, string: *const Char16) -> Status,
    pub _test_string: extern "win64" fn(this: &SimpleTextOutput, string: *const Char16) -> Status,
    pub _query_mode: extern "win64" fn(
        this: &SimpleTextOutput,
        mode_number: usize,
        columns: &mut usize,
        rows: &mut usize
    ) -> Status,
    pub _set_mode: extern "win64" fn(this: &SimpleTextOutput, mode_number: usize) -> Status,
    pub _set_attribute: extern "win64" fn(this: &SimpleTextOutput, attribute: usize) -> Status,
    pub _clear_screen: extern "win64" fn(this: &SimpleTextOutput) -> Status,
    pub _set_cursor_position: extern "win64" fn(
        this: &SimpleTextOutput,
        column: usize,
        row: usize
    ) -> Status,
    pub _enable_cursor: extern "win64" fn(this: &SimpleTextOutput, visible: Bool) -> Status,
    pub mode: OwnedPtr<SimpleTextOutputMode>,
}

impl SimpleTextOutput {

    /// Reset the ConsoleOut device
    pub fn reset(&self, extended_verification: bool) -> Result<(), Status> {

        (self._reset)(self, Bool::from(extended_verification))
            .as_result()?;
        Ok(())
    }

    /// Displays the string on the device at the current cursor location
    pub fn output_string(&self, string: &str) -> Result<(), Status> {

        exec_with_str(string, |buf| (self._output_string)(self, buf))
    }

    /// Tests to see if the ConsoleOut device supports this string
    pub fn test_string(&self, string: &str) -> Result<(), Status> {

        exec_with_str(string, |buf| (self._test_string)(self, buf))
    }

    /// Queries information concerning the output device's supported text mode
    pub fn query_mode(&self, mode_number: usize) -> Result<ModeDescriptor, Status> {

        let mut desc = ModeDescriptor::default();
        (self._query_mode)(self, mode_number, &mut desc.columns, &mut desc.rows)
            .as_result()
            .map(|_| desc)
    }

    /// Sets the current mode of the output device
    pub fn set_mode(&self, mode_number: usize) -> Result<(), Status> {

        (self._set_mode)(self, mode_number)
            .as_result()?;
        Ok(())
    }

    /// Sets the foreground and background color of the text that is output
    pub fn set_attribute(&self, foreground: Color, background: Color) -> Result<(), Status> {

        // TODO: is this necessary, or will the implementation correctly report an error?
        if !background.is_background() {
            return Err(Status::InvalidParameter);
        }

        let attribute = ((background as usize) << 4) | (foreground as usize);

        (self._set_attribute)(self, attribute)
            .as_result()?;
        Ok(())
    }

    /// Clears the screen with the currently set background color
    pub fn clear_screen(&self) -> Result<(), Status> {

        (self._clear_screen)(self)
            .as_result()?;
        Ok(())
    }

    /// Sets the current cursor position
    pub fn set_cursor_position(&self, column: usize, row: usize) -> Result<(), Status> {

        (self._set_cursor_position)(self, column, row)
            .as_result()?;
        Ok(())
    }

    /// Turns the visibility of the cursor on/off
    pub fn enable_cursor(&self, visible: bool) -> Result<(), Status> {

        (self._enable_cursor)(self, Bool::from(visible))
            .as_result()?;
        Ok(())
    }
}


/// Converts string to Char16 and calls the given function
///
/// The UEFI spec represents strings using UTF-16, so Rust's `&str` type is not directly compatible.
/// This function properly converts a `&str` to UTF-16, then calls the given function with a pointer
/// to the UTF-16 string.
///
/// Since this is UEFI, there is no dynamic allocation, so the conversion actually happens 127
/// characters at a time using a stack-allocated buffer. Because of this, `f` may actually be called
/// more than one time, once for every 127 characters.
///
/// TODO:
///
/// * if dynamic allocation becomes possible, this function might not be necessary
fn exec_with_str<F>(string: &str, f: F) -> Result<(), Status>
    where F: Fn(*const Char16) -> Status {

    // Allocate a buffer to encode the string piece by piece (can't do it all at once since
    // there is no dynamic allocation in this environment)
    const BUFSIZE: usize = 128;
    let mut buf: [u16; BUFSIZE] = [0u16; BUFSIZE];
    let mut i = 0;

    // Interpret the string as UTF-16 and fill the buffer
    for c in string.chars() {
        let mut char_buf: [u16; 2] = [0u16; 2];
        buf[i] = c.encode_utf16(&mut char_buf)[0]; // TODO: this drops the second character
        i += 1;

        if i == BUFSIZE - 1 {
            // Write out the string
            // BUFSIZE - 1 ensures at least one null character is present
            // TODO: what if this returns an error code?
            f(buf.as_ptr())
                .as_result()?;

            // Fill the buffer back up with null characters
            for j in 0..BUFSIZE {
                buf[j] = 0u16;
            }

            // Finally, reset our iterator
            i = 0;
        }
    }

    // Flush whatever remains in the buffer
    if i != 0 {
        f(buf.as_ptr())
            .as_result()?;
    }

    Ok(())
}


/// Describes the current attributes of the output device
#[repr(C)]
pub struct SimpleTextOutputMode {
    pub max_mode: i32,
    pub mode: i32,
    pub attribute: i32,
    pub cursor_column: i32,
    pub cursor_row: i32,
    pub cursor_visible: Bool,
}


/// Describes the dimensions of an output device mode
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct ModeDescriptor {
    pub columns: usize,
    pub rows: usize,
}
