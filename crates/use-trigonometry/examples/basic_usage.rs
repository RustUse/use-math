use use_trigonometry::{
    Angle, cos_deg, degrees_to_radians, normalize_degrees, radians_to_degrees, sin_deg, tan_deg,
};

fn main() {
    let acute = Angle::from_degrees(30.0);
    let wrapped = Angle::from_degrees(765.0).normalized();
    let signed = Angle::from_degrees(-450.0).normalized_signed();

    assert!((acute.radians() - degrees_to_radians(30.0)).abs() < 1.0e-12);
    assert!((acute.degrees() - radians_to_degrees(acute.radians())).abs() < 1.0e-12);
    assert!((acute.sin() - 0.5).abs() < 1.0e-12);
    assert!((acute.cos() - cos_deg(30.0)).abs() < 1.0e-12);
    assert!((wrapped.degrees() - 45.0).abs() < 1.0e-12);
    assert!((signed.degrees() + 90.0).abs() < 1.0e-12);
    assert!((normalize_degrees(-90.0) - 270.0).abs() < 1.0e-12);
    assert!((sin_deg(30.0) - 0.5).abs() < 1.0e-12);
    assert!((tan_deg(45.0) - 1.0).abs() < 1.0e-12);
}
