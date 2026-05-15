use use_arithmetic::{
    checked_add, div_floor, gcd, is_divisible_by, is_even, is_odd, lcm, mod_floor, saturating_add,
    wrapping_mul,
};

fn main() {
    assert_eq!(gcd(54, 24), 6);
    assert_eq!(lcm(6, 15), 30);
    assert!(is_divisible_by(84, 7));
    assert!(is_even(12));
    assert!(is_odd(7));
    assert_eq!(div_floor(-7, 3), -3);
    assert_eq!(mod_floor(-7, 3), 2);
    assert_eq!(checked_add(u8::MAX, 1), None);
    assert_eq!(saturating_add(u8::MAX, 1), u8::MAX);
    assert_eq!(wrapping_mul(200_u8, 2), 144);
}
