use crate::{Axis, MAT4_IDENTITY};
use glam::{Mat4, Vec4};

pub fn calculate_rotation_matrix(angle: f32, axis: Axis) -> Mat4 {
    if angle == 0.0 {
        return *MAT4_IDENTITY;
    };

    let angle_sin = angle.sin();
    let angle_cos = angle.cos();
    match axis {
        Axis::X => Mat4::from_cols(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, angle_cos, -angle_sin, 0.0),
            Vec4::new(0.0, angle_sin, angle_cos, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ),
        Axis::Y => Mat4::from_cols(
            Vec4::new(angle_cos, 0.0, angle_sin, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(-angle_sin, 0.0, angle_cos, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ),
        Axis::Z => Mat4::from_cols(
            Vec4::new(angle_cos, -angle_sin, 0.0, 0.0),
            Vec4::new(angle_sin, angle_cos, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::common::deg_to_rad;

    use super::*;

    #[test]
    fn test_rotation_matrix() {
        let rad_45 = deg_to_rad(45.0);
        assert_eq!(
            calculate_rotation_matrix(rad_45, Axis::X),
            Mat4::from_cols(
                Vec4::new(1.0, 0.0, 0.0, 0.0),
                Vec4::new(0.0, 0.70710677, -0.70710677, 0.0),
                Vec4::new(0.0, 0.70710677, 0.70710677, 0.0),
                Vec4::new(0.0, 0.0, 0.0, 1.0),
            )
        );
        assert_eq!(
            calculate_rotation_matrix(rad_45, Axis::Y),
            Mat4::from_cols(
                Vec4::new(0.70710677, 0.0, 0.70710677, 0.0),
                Vec4::new(0.0, 1.0, 0.0, 0.0),
                Vec4::new(-0.70710677, 0.0, 0.70710677, 0.0),
                Vec4::new(0.0, 0.0, 0.0, 1.0),
            )
        );
        assert_eq!(calculate_rotation_matrix(0.0, Axis::Z), *MAT4_IDENTITY);
    }
}
