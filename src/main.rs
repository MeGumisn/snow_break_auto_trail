use window_util::get_window_handle;
use windows::Win32::UI::WindowsAndMessaging::SetProcessDPIAware;

mod capture;
mod dxgi;
mod window_util;

fn main() {
    // get the window handle (hwnd) from your application context
    unsafe {
        let _ = SetProcessDPIAware();
    }
    let dxgi = dxgi::Dxgi::new();
    let title = "MuMu模拟器12";
    let hwnd = get_window_handle(title);

    dxgi.init_dxgi(hwnd);
    println!("{}", "cur");
    let capture = capture::DxgiCapture::new(dxgi);
    println!("{}", "cur");
    let client_rect = window_util::get_client_size(hwnd);
    // let window_rect = window_util::get_window_size(hwnd);
    println!("{}, {}, {}, {}", client_rect.0, client_rect.1, client_rect.2, client_rect.3);
    let (mat, _vec) = capture.capture_rect(client_rect);
    println!("{}", "cur");
    capture.show(&mat);
    capture.destory();
    // Process the result as needed
    println!("Hello, world!");
}
