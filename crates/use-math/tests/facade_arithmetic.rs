use use_math::{checked_add, div_floor, mod_floor, saturating_add, wrapping_mul};

#[test]
fn facade_root_reexports_non_conflicting_arithmetic_workflow() {
    assert_eq!(checked_add(u8::MAX, 1), None);
    assert_eq!(div_floor(-7, 3), -3);
    assert_eq!(mod_floor(-7, 3), 2);
    assert_eq!(saturating_add(u8::MAX, 1), u8::MAX);
    assert_eq!(wrapping_mul(200_u8, 2), 144);
}

#[test]
fn facade_namespace_reexports_full_arithmetic_surface() {
    assert_eq!(use_math::arithmetic::gcd(54, 24), 6);
    assert_eq!(use_math::arithmetic::lcm(6, 15), 30);
    assert!(use_math::arithmetic::is_divisible_by(84, 7));
    assert!(use_math::arithmetic::is_even(12));
    assert!(use_math::arithmetic::is_odd(7));
    assert_eq!(
        use_math::arithmetic::checked_lcm(u64::MAX, u64::MAX - 1),
        None
    );
}
