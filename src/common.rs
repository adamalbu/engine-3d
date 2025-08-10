use glam::{Mat4, Vec4};
use std::{f32::consts::PI, sync::LazyLock};

pub static MAT4_IDENTITY: LazyLock<Mat4> = LazyLock::new(|| {
    Mat4::from_cols(
        Vec4::new(1.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 1.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    )
});

pub enum Axis {
    X,
    Y,
    Z,
}

pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * (PI / 180.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deg_to_rad;

    #[test]
    fn test_deg_to_rad() {
        assert_eq!(deg_to_rad(180.0), PI);
        assert_eq!(deg_to_rad(45.0), PI / 4.0);
        assert_eq!(deg_to_rad(360.0), PI * 2.0);
    }
}
