//! Small numeric constants that are reused across the workspace.

/// The golden ratio as an `f64`.
pub const GOLDEN_RATIO: f64 = 1.618_033_988_749_895;

/// The golden ratio as an `f32`.
pub const GOLDEN_RATIO_F32: f32 = 1.618_034;

/// The square root of three as an `f64`.
pub const SQRT_3: f64 = 1.732_050_807_568_877_2;

/// The square root of three as an `f32`.
pub const SQRT_3_F32: f32 = 1.732_050_8;

#[cfg(test)]
mod tests {
    use super::{GOLDEN_RATIO, GOLDEN_RATIO_F32, SQRT_3, SQRT_3_F32};

    #[test]
    fn constants_match_their_reference_relationships() {
        assert!(
            GOLDEN_RATIO
                .mul_add(GOLDEN_RATIO, -(GOLDEN_RATIO + 1.0))
                .abs()
                < 1.0e-12
        );
        assert!(
            GOLDEN_RATIO_F32
                .mul_add(GOLDEN_RATIO_F32, -(GOLDEN_RATIO_F32 + 1.0))
                .abs()
                < 1.0e-5
        );
        assert!(SQRT_3.mul_add(SQRT_3, -3.0).abs() < 1.0e-12);
        assert!(SQRT_3_F32.mul_add(SQRT_3_F32, -3.0).abs() < 1.0e-5);
    }
}
