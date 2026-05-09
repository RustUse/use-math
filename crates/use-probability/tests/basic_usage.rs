use use_probability::{
    Bernoulli, Probability, ProbabilityError, independent_intersection, independent_union,
};

fn assert_close(left: f64, right: f64, tolerance: f64) {
    assert!(
        (left - right).abs() <= tolerance,
        "expected {left} to be within {tolerance} of {right}"
    );
}

#[test]
fn direct_probability_usage_covers_probability_rules() -> Result<(), ProbabilityError> {
    let success = Probability::from_fraction(1, 4)?;
    let other = Probability::try_new(0.5)?;
    let bernoulli = Bernoulli::new(success);

    let joint = independent_intersection(success, other);
    let either = independent_union(success, other);

    assert_close(joint.value(), 0.125, 1.0e-12);
    assert_close(either.value(), 0.625, 1.0e-12);
    assert_eq!(bernoulli.pmf(true), success);
    assert_eq!(bernoulli.failure_probability(), Probability::try_new(0.75)?);
    assert_close(bernoulli.mean(), 0.25, 1.0e-12);
    assert_close(bernoulli.variance(), 0.1875, 1.0e-12);

    Ok(())
}

#[test]
fn probability_validation_rejects_invalid_values() {
    assert!(matches!(
        Probability::try_new(-0.1),
        Err(ProbabilityError::ProbabilityOutOfRange(_))
    ));
    assert!(matches!(
        Probability::from_fraction(2, 1),
        Err(ProbabilityError::PartExceedsTotal { .. })
    ));
}
