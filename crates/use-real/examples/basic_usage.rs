use use_real::{Real, RealInterval, approx_eq};

fn main() -> Result<(), use_real::RealError> {
	let interval = RealInterval::try_new(-2.0, 6.0)?;
	let value = Real::try_new(8.0)?;
	let midpoint = interval.midpoint();
	let clamped = interval.clamp(value);

	assert!((midpoint.value() - 2.0).abs() < 1.0e-12);
	assert_eq!(clamped, Real::try_new(6.0)?);
	assert!(approx_eq(midpoint, Real::try_new(2.0)?, 1.0e-12)?);

	Ok(())
}
