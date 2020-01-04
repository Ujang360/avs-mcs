use crate::{get_camera_matrix, get_distortion_coefficients, get_object_points};
use mcslib_common::types::SafePoint2D;
use opencv::calib3d::{rodrigues, solve_pnp};
use opencv::core::{no_array, DataType, Mat, Point2d, _OutputArrayTrait};
use opencv::prelude::Vector;
use opencv::types::VectorOfPoint2d;
use opencv::Error as OpenCVError;

pub fn compute_pose(
    image_points_array: &[SafePoint2D],
    translation: &mut [f64; 3],
    euler_angles: &mut [f64; 3],
) -> Result<(), OpenCVError> {
    let object_points_ref = get_object_points();
    let camera_matrix = get_camera_matrix();
    let dist_coeffs = get_distortion_coefficients();

    unsafe {
        let mut image_points = VectorOfPoint2d::new();

        for image_point_ref in image_points_array {
            image_points.push(Point2d::new(image_point_ref.x, image_point_ref.y));
        }

        let mut rotation_vector: Mat = Mat::default().unwrap();
        let mut translation_vector: Mat = Mat::default().unwrap();
        solve_pnp(
            object_points_ref,
            &image_points,
            camera_matrix,
            dist_coeffs,
            &mut rotation_vector,
            &mut translation_vector,
            false,
            0,
        )?;
        let mut no_array = no_array().unwrap();
        rodrigues(&rotation_vector, &mut translation_vector, &mut no_array)?;
        let rv_val_0_2 = *rotation_vector.at_2d::<f64>(0, 2).unwrap();
        let rv_val_1_0 = *rotation_vector.at_2d::<f64>(1, 0).unwrap();
        let rv_val_1_1 = *rotation_vector.at_2d::<f64>(1, 1).unwrap();
        let rv_val_1_2 = *rotation_vector.at_2d::<f64>(1, 2).unwrap();
        let rv_val_2_2 = *rotation_vector.at_2d::<f64>(2, 2).unwrap();
        let mut dst = Mat::new_rows_cols(3, 1, f64::typ()).unwrap();
        *dst.at_2d_mut::<f64>(0, 0).unwrap() = rv_val_1_2.asin().to_degrees();
        *dst.at_2d_mut::<f64>(1, 0).unwrap() = rv_val_0_2.atan2(rv_val_2_2).to_degrees();
        *dst.at_2d_mut::<f64>(2, 0).unwrap() = (-rv_val_1_0.atan2(rv_val_1_1)).to_degrees();
        translation[0] = translation_vector.at_3d::<f64>(2, 0, 0).unwrap().round() / 10.0;
        translation[1] = translation_vector.at_3d::<f64>(0, 0, 0).unwrap().round() / 10.0;
        translation[2] = translation_vector.at_3d::<f64>(1, 0, 0).unwrap().round() / 10.0;
        euler_angles[0] = dst.at_2d::<f64>(2, 0).unwrap().round();
        euler_angles[1] = dst.at_2d::<f64>(0, 0).unwrap().round();
        euler_angles[2] = dst.at_2d::<f64>(1, 0).unwrap().round();

        let _ = rotation_vector.release();
        let _ = translation_vector.release();
        let _ = no_array.release();
        let _ = dst.release();

        Ok(())
    }
}
