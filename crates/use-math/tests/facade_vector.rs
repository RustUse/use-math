use use_math::vector;

fn approx_eq(left: f64, right: f64) -> bool {
    (left - right).abs() < 1.0e-12
}

#[test]
fn facade_namespace_reexports_vector_workflow() {
    let displacement = vector::Vector2::new(3.0, 4.0);
    let x = vector::Vector3::new(1.0, 0.0, 0.0);
    let y = vector::Vector3::new(0.0, 1.0, 0.0);
    let midpoint = vector::Vector4::ZERO.lerp(vector::Vector4::new(4.0, 8.0, 12.0, 16.0), 0.5);

    assert!(approx_eq(displacement.magnitude(), 5.0));
    assert!(approx_eq(vector::Vector2::ZERO.distance(displacement), 5.0));
    assert_eq!(x.cross(y), vector::Vector3::new(0.0, 0.0, 1.0));
    assert_eq!(
        vector::Vector3::ONE.scale(2.0),
        vector::Vector3::new(2.0, 2.0, 2.0)
    );
    assert_eq!(midpoint, vector::Vector4::new(2.0, 4.0, 6.0, 8.0));
}
