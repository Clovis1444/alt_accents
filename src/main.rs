// main.rs

mod accent;
mod data;
mod hook;
mod window;

use windows::{core::*, Win32::UI::WindowsAndMessaging::*};
fn main() -> Result<()> {
    println!("{}", data::get_accent(data::AccentKey::E, false, 3));

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
