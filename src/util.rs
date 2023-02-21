use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::HWND;

pub(crate) fn get_hwnd_from_raw_window_handle<W: HasRawWindowHandle>(window: &W) -> HWND {
    // Get the window handle
    let window_handle = window.raw_window_handle();
    let window_handle = match window_handle {
        RawWindowHandle::Win32(window_handle) => window_handle.hwnd,
        _ => panic!("Unsupported platform!"),
    };
    HWND(window_handle as isize)
}
