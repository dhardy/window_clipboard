use crate::ClipboardProvider;

use raw_window_handle::HasRawWindowHandle;
use std::error::Error;

pub fn new_clipboard<W: HasRawWindowHandle>(
    _window: &W,
) -> Result<Box<dyn ClipboardProvider>, Box<dyn Error>> {
    Ok(Box::new(clipboard_macos::Clipboard::new()?))
}

impl ClipboardProvider for clipboard_macos::Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        self.read()
    }

    fn write(
        &mut self,
        string: std::borrow::Cow<str>,
    ) -> Result<(), Box<dyn Error>> {
        self.write(string.to_string())
    }
}
