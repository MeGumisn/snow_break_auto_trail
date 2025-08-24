use crate::dxgi::Dxgi;
use opencv::core::{Mat, Mat_AUTO_STEP, Size, CV_8UC1, CV_8UC3, CV_8UC4};
use opencv::highgui;
use std::ffi::c_void;
use opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT;
use opencv::imgproc::cvt_color;

pub struct DxgiCapture {
    dxgi: Dxgi,
}

impl DxgiCapture {
    pub fn new(dxgi: Dxgi) -> DxgiCapture {
        DxgiCapture { dxgi }
    }

    pub fn capture_rect(&self, rect: (i32, i32, i32, i32)) -> Mat {
        let (_left, _top, width, height) = rect;
        // 这里vec和res用了同一块内存,所以得返回出去
        let vec = vec![0u8; (width * height * 4) as usize];
        self.dxgi.grab(&vec, _left, _top, width, height);
        let mat = unsafe {
            Mat::new_size_with_data_unsafe(
                Size::new(width, height),
                CV_8UC4,
                vec.as_ptr() as *mut c_void,
                Mat_AUTO_STEP,
            )
        }
        .unwrap();
        let mut bgr_mat = unsafe{Mat::new_size(Size::new(width, height), CV_8UC3)}.unwrap();
        let _ = cvt_color(&mat, &mut bgr_mat, opencv::imgproc::COLOR_BGRA2BGR, 0, ALGO_HINT_DEFAULT);
        bgr_mat
    }

    pub fn capture_rect_gray(&self, rect: (i32, i32, i32, i32))->Mat{
        let (_left, _top, width, height) = rect;
        // 这里vec和res用了同一块内存,所以得返回出去
        let vec = vec![0u8; (width * height * 4) as usize];
        self.dxgi.grab(&vec, _left, _top, width, height);
        let mat = unsafe {
            Mat::new_size_with_data_unsafe(
                Size::new(width, height),
                CV_8UC4,
                vec.as_ptr() as *mut c_void,
                Mat_AUTO_STEP,
            )
        }
            .unwrap();
        let mut gray_mat = unsafe{Mat::new_size(Size::new(width, height), CV_8UC1)}.unwrap();
        let _ = cvt_color(&mat, &mut gray_mat, opencv::imgproc::COLOR_BGRA2GRAY, 0, ALGO_HINT_DEFAULT);
        gray_mat
    }


    pub fn show(&self, mat: &Mat) -> () {
        // 显示图像
        highgui::imshow("Screen Capture", mat).expect("Failed to show image");
        highgui::wait_key(0).expect("wait_key failed");
    }

    pub fn destroy(&self) {
        self.dxgi.destroy()
    }
}
