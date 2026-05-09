use use_rational::Rational;

fn main() -> Result<(), use_rational::RationalError> {
	let half = Rational::try_new(1, 2)?;
	let third = Rational::try_new(1, 3)?;
	let sum = half.checked_add(third)?;
	let quotient = half.checked_div(third)?;

	assert_eq!(sum, Rational::try_new(5, 6)?);
	assert_eq!(quotient, Rational::try_new(3, 2)?);
	assert!((sum.as_f64() - (5.0 / 6.0)).abs() < 1.0e-12);

	Ok(())
}
