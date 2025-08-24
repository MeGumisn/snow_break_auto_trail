mod common;
mod graphic;
mod task;
mod utils;

use graphic::{capture, dxgi};
use utils::window_util;
use windows::Win32::UI::WindowsAndMessaging::SetProcessDPIAware;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        // get the window handle (hwnd) from your application context
        unsafe {
            let _ = SetProcessDPIAware();
        }
        let dxgi = dxgi::Dxgi::new();
        let title = "尘白禁区";
        let hwnd = window_util::get_window_handle(title);

        dxgi.init_dxgi(hwnd);
        println!("{}", "init dxgi lib success.");
        let capture = capture::DxgiCapture::new(dxgi);
        println!("{}", "create capture success.");
        let mut client_rect = window_util::get_client_size(hwnd);
        // let window_rect = window_util::get_window_size(hwnd);
        println!("client rect: {client_rect:?}");

        let window_rect = window_util::get_window_size(hwnd);
        println!("window rect: {window_rect:?}");
        client_rect.1 = ((31f64) * 1.5) as i32;
        println!("capture rect: {client_rect:?}");
        let mat = capture.capture_rect(client_rect);
        println!("capture client rect(BGR) success");
        capture.show(&mat);

        let gray_mat = capture.capture_rect_gray(client_rect);

        println!("capture client rect(GRAY) success");
        capture.show(&gray_mat);
        capture.destroy();
        // Process the result as needed
        println!("It just works!!!");
    }
}
fn main() {}
