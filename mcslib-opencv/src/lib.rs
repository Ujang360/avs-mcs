pub extern crate opencv;

pub mod motion_tracker_compute;

use mcslib_common::once_cell::unsync::OnceCell;
use opencv::core::{DataType, Mat, MatExpr, Point3d};
use opencv::prelude::Vector;
use opencv::types::VectorOfPoint3d;

pub const CAMERA_WIDTH: i32 = 10;
pub const CAMERA_RADIUS: f64 = CAMERA_WIDTH as f64 / 2.0;
pub const FOCAL_LENGTH: f64 = CAMERA_WIDTH as f64;
pub const CAMERA_MATRIX_ARRAY: [[f64; 3]; 3] = [
    [FOCAL_LENGTH, 0.0, CAMERA_RADIUS],
    [FOCAL_LENGTH, CAMERA_RADIUS, 0.0],
    [0.0, 0.0, 1.0],
];

static mut INIT_STATE: bool = false;
static mut CAMERA_MATRIX: OnceCell<Mat> = OnceCell::new();
static mut DISTORTION_COEFFICIENTS: OnceCell<MatExpr> = OnceCell::new();
static mut REFERENCE_POINTS: OnceCell<VectorOfPoint3d> = OnceCell::new();

pub fn is_initialized() -> bool {
    unsafe { INIT_STATE }
}

pub fn init_globals() {
    if is_initialized() {
        return;
    }

    unsafe {
        CAMERA_MATRIX.get_or_init(|| {
            let mut camera_matrix = Mat::new_rows_cols(3, 3, f64::typ()).unwrap();

            for row in 0..3 {
                for col in 0..3 {
                    *camera_matrix.at_2d_mut(row, col).unwrap() = CAMERA_MATRIX_ARRAY[row as usize][col as usize];
                }
            }

            camera_matrix
        });
        DISTORTION_COEFFICIENTS.get_or_init(|| Mat::zeros(4, 1, f64::typ()).unwrap());
        REFERENCE_POINTS.get_or_init(|| {
            let mut ref_points = VectorOfPoint3d::new();
            ref_points.push(Point3d::new(11.4, 0.0, -11.4));
            ref_points.push(Point3d::new(11.4, 0.0, 11.4));
            ref_points.push(Point3d::new(-11.4, 0.0, 11.4));
            ref_points.push(Point3d::new(-11.4, 0.0, -11.4));
            ref_points
        });
    }
}

pub fn get_camera_matrix<'a>() -> &'a Mat {
    unsafe { CAMERA_MATRIX.get().unwrap() }
}

pub fn get_distortion_coefficients<'a>() -> &'a MatExpr {
    unsafe { DISTORTION_COEFFICIENTS.get().unwrap() }
}

pub fn get_object_points<'a>() -> &'a VectorOfPoint3d {
    unsafe { REFERENCE_POINTS.get().unwrap() }
}
