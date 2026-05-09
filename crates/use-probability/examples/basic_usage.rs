use use_probability::{Bernoulli, Probability, independent_union};

fn main() -> Result<(), use_probability::ProbabilityError> {
    let rain = Probability::from_fraction(1, 4)?;
    let traffic = Probability::try_new(0.5)?;
    let either_delay = independent_union(rain, traffic);
    let commute = Bernoulli::new(rain);

    assert!((either_delay.value() - 0.625).abs() < 1.0e-12);
    assert_eq!(commute.failure_probability(), Probability::try_new(0.75)?);
    assert!((commute.variance() - 0.1875).abs() < 1.0e-12);

    Ok(())
}
