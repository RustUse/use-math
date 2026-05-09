use use_integer::{IntegerSign, are_coprime, classify_sign, gcd, is_divisible_by, lcm};

fn main() -> Result<(), use_integer::IntegerError> {
	assert_eq!(classify_sign(-42), IntegerSign::Negative);
	assert!(is_divisible_by(42, 6)?);
	assert!(are_coprime(35, 64));
	assert_eq!(gcd(-54, 24), 6);
	assert_eq!(lcm(-6, 15)?, 30);

	Ok(())
}
