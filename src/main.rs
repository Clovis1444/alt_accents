// main.rs

mod accent;
mod data;
mod hook;
#[cfg(test)]
mod tests;
mod window;

use windows::{core::*, Win32::UI::WindowsAndMessaging::*};
fn main() -> Result<()> {
    unsafe {
        let _hwnd = match window::create_window() {
            Err(_) => panic!("Failed to create a window!"),
            Ok(handle) => handle,
        };

        let hhk = hook::setup_hook();

        // Message buffer
        let mut message = MSG::default();

        // Main message loop
        // Get messages from OS and dispatch them
        while GetMessageW(&mut message, None, 0, 0).into() {
            DispatchMessageW(&message);
            TranslateMessage(&message);
        }

        hook::remove_hook(hhk);
        Ok(())
    }
}
