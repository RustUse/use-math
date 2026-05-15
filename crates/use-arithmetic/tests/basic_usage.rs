use use_arithmetic::{
    checked_add, checked_div_floor, checked_is_divisible_by, checked_lcm, checked_mod_floor,
    checked_mul, checked_sub, div_ceil, div_floor, gcd, is_divisible_by, is_even, is_odd, lcm,
    mod_floor, saturating_add, saturating_mul, saturating_sub, wrapping_add, wrapping_mul,
    wrapping_sub,
};

#[test]
fn direct_arithmetic_usage_covers_core_workflow() {
    assert_eq!(gcd(54, 24), 6);
    assert_eq!(gcd(0, 0), 0);
    assert_eq!(lcm(6, 15), 30);
    assert_eq!(lcm(0, 15), 0);
    assert!(is_divisible_by(84, 7));
    assert!(!is_divisible_by(84, 0));
    assert!(is_even(12));
    assert!(is_odd(7));
    assert_eq!(div_floor(-7, 3), -3);
    assert_eq!(div_ceil(-7, 3), -2);
    assert_eq!(mod_floor(-7, 3), 2);
}

#[test]
fn checked_helpers_surface_failure_cases() {
    assert_eq!(checked_lcm(6, 15), Some(30));
    assert_eq!(checked_lcm(u64::MAX, u64::MAX - 1), None);
    assert_eq!(checked_is_divisible_by(84, 7), Some(true));
    assert_eq!(checked_is_divisible_by(84, 0), None);
    assert_eq!(checked_div_floor(-7, 3), Some(-3));
    assert_eq!(checked_div_floor(7, 0), None);
    assert_eq!(checked_mod_floor(7, 0), None);
    assert_eq!(checked_add(u8::MAX, 1), None);
    assert_eq!(checked_sub(0_u8, 1), None);
    assert_eq!(checked_mul(200_u8, 2), None);
}

#[test]
fn alternate_overflow_modes_remain_explicit() {
    assert_eq!(saturating_add(u8::MAX, 1), u8::MAX);
    assert_eq!(saturating_sub(0_u8, 1), 0);
    assert_eq!(saturating_mul(200_u8, 2), u8::MAX);
    assert_eq!(wrapping_add(u8::MAX, 1), 0);
    assert_eq!(wrapping_sub(0_u8, 1), u8::MAX);
    assert_eq!(wrapping_mul(200_u8, 2), 144);
}
