use use_math::{checked_add, div_floor, mod_floor, saturating_add, wrapping_mul};

fn main() {
    assert_eq!(checked_add(u8::MAX, 1), None);
    assert_eq!(div_floor(-7, 3), -3);
    assert_eq!(mod_floor(-7, 3), 2);
    assert_eq!(saturating_add(u8::MAX, 1), u8::MAX);
    assert_eq!(wrapping_mul(200_u8, 2), 144);
    assert_eq!(use_math::arithmetic::gcd(54, 24), 6);
    assert_eq!(use_math::arithmetic::lcm(6, 15), 30);
    assert!(use_math::arithmetic::is_even(12));
}
