use crate::capture::DxgiCapture;
use opencv::core::{Mat, MatTraitConst, Point, min_max_loc, no_array};
use opencv::imgproc::{TM_CCOEFF_NORMED, match_template, match_template_def};
use windows::Win32::Foundation::HWND;

struct TrailTask {
    pub hwnd: HWND,
    pub capture: DxgiCapture,
}

impl TrailTask {
    pub fn check_match_templates(gray_img: Mat, gray_template: Mat) -> Option<Point> {
        let mut result = Mat::default();
        match_template(
            &gray_img,
            &gray_template,
            &mut result,
            TM_CCOEFF_NORMED,
            &no_array(),
        )
        .expect("Failed to match template");
        let mut min_val = 0.0;
        let mut max_val = 0.0;
        let mut min_loc = Point { x: 0, y: 0 };
        let mut max_loc = Point { x: 0, y: 0 };
        min_max_loc(
            &result,
            Some(&mut min_val),
            Some(&mut max_val),
            Some(&mut min_loc),
            Some(&mut max_loc),
            &no_array(),
        )
        .expect("Failed to match template");
        if max_val > 0.8 {
            let t_width = gray_template.cols();
            let t_height = gray_template.rows();
            Some(Point {
                x: max_loc.x + t_width / 2,
                y: max_loc.y + t_height / 2,
            })
        } else {
            None
        }
    }
}
