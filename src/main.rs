use window_util::get_window_handle;
use windows::Win32::UI::WindowsAndMessaging::SetProcessDPIAware;

mod capture;
mod dxgi;
mod window_util;
mod settings;

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
        let title = "MuMu模拟器12";
        let hwnd = get_window_handle(title);

        dxgi.init_dxgi(hwnd);
        println!("{}", "init dxgi lib success.");
        let capture = capture::DxgiCapture::new(dxgi);
        println!("{}", "create capture success.");
        let client_rect = window_util::get_client_size(hwnd);
        // let window_rect = window_util::get_window_size(hwnd);
        println!(
            "client rect: {}, {}, {}, {}",
            client_rect.0, client_rect.1, client_rect.2, client_rect.3
        );

        let window_rect = window_util::get_window_size(hwnd);
        println!(
            "window rect: {}, {}, {}, {}",
            window_rect.0, window_rect.1, window_rect.2, window_rect.3
        );
        let mat = capture.capture_rect(client_rect);
        println!("{}", "capture client rect(BGR) success");
        capture.show(&mat);

        let gray_mat = capture.capture_rect_gray(client_rect);

        println!("{}", "capture client rect(GRAY) success");
        capture.show(&gray_mat);
        capture.destory();
        // Process the result as needed
        println!("It just works!!!");
    }
}
fn main() {}
