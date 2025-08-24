use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetClientRect, GetWindowRect};
pub fn get_window_handle(title: &str) -> HWND {
    let mut window_title: Vec<u16> = title.encode_utf16().collect();
    window_title.push(0);
    println!("window_title: {:?}", window_title);
    unsafe {
        FindWindowW(None, PCWSTR::from_raw(window_title.as_ptr())).expect("Failed to find window")
    }
}

pub fn get_client_size(hwnd: HWND) -> (i32, i32, i32, i32) {
    unsafe {
        let mut rect = RECT::default();
        GetClientRect(hwnd, &mut rect).expect("Failed to get window rectangle.");
        (
            rect.left,
            rect.top,
            rect.right - rect.left,
            rect.bottom - rect.top,
        )
    }
}

pub fn get_window_size(hwnd: HWND) -> (i32, i32, i32, i32) {
    unsafe {
        let mut rect = RECT::default();
        GetWindowRect(hwnd, &mut rect).expect("Failed to get window rectangle.");
        (
            rect.left,
            rect.top,
            rect.right - rect.left,
            rect.bottom - rect.top,
        )
    }
}
