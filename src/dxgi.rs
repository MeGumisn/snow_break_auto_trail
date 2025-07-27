#![allow(non_snake_case)]

use std::ffi::c_int;
use libloading::Library;
use windows::Win32::Foundation::HWND;

pub struct Dxgi {
    lib: Library,
}
impl Dxgi {
    pub fn new() -> Dxgi {
        let inner_lib = unsafe { Library::new("dxgi4py.dll") }.expect("Failed to load dxgi4py.dll");
        Self { lib: inner_lib }
    }

    pub fn init_dxgi(&self, hwnd: HWND) -> () {
        Ok::<(), &str>(unsafe {
            let init_dxgi = self
                .lib
                .get::<extern "C" fn(HWND) -> ()>(b"init_dxgi\0")
                .expect("Failed to get init_dxgi")
                .into_raw();
            init_dxgi(hwnd);
        })
        .expect("Failed to init dxgi")
    }

    pub fn grab(
        &self,
        shot_data: &[u8],
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
    ) -> *const u8 {
        unsafe {
            let shot_ptr = shot_data.as_ptr();
            let grab = self
                .lib
                .get::<extern "C" fn(*const u8, c_int, c_int, c_int, c_int) -> *const u8>(b"grab\0")
                .expect("Failed to get grab")
                .into_raw();
            grab(shot_ptr, left, top, width, height)
        }
    }

    pub fn destroy(&self) -> () {
        unsafe {
            let destroy = self
                .lib
                .get::<extern "C" fn() -> ()>(b"destroy\0")
                .expect("Failed to get destroy")
                .into_raw();
            destroy();
        }
    }
}
