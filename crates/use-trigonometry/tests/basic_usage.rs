use core::f64::consts::{PI, TAU};
use use_trigonometry::{
    Angle, cos_deg, degrees_to_radians, normalize_degrees, normalize_radians, radians_to_degrees,
    sin_deg, tan_deg,
};

fn assert_close(left: f64, right: f64) {
    assert!((left - right).abs() < 1.0e-12, "left={left}, right={right}");
}

#[test]
fn direct_trigonometry_usage_covers_angle_helpers() {
    let acute = Angle::from_degrees(30.0);
    let wrapped = Angle::from_degrees(-30.0).normalized();

    assert_close(acute.radians(), PI / 6.0);
    assert_close(radians_to_degrees(acute.radians()), 30.0);
    assert_close(degrees_to_radians(90.0), PI / 2.0);
    assert_close(wrapped.degrees(), 330.0);
    assert_close(normalize_degrees(-90.0), 270.0);
    assert_close(normalize_radians(-PI / 2.0), TAU - (PI / 2.0));
    assert_close(acute.sin(), 0.5);
    assert_close(cos_deg(60.0), 0.5);
    assert_close(sin_deg(30.0), 0.5);
    assert_close(tan_deg(45.0), 1.0);
}
