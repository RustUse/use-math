use use_calculus::{
    CalculusError, Differentiator, IntegrationInterval, Integrator, LimitApproximator,
    symmetric_limit,
};

fn assert_close(left: f64, right: f64, tolerance: f64) {
    assert!(
        (left - right).abs() <= tolerance,
        "expected {left} to be within {tolerance} of {right}"
    );
}

#[test]
fn direct_calculus_crate_usage_covers_derivatives_integrals_and_limits() -> Result<(), CalculusError>
{
    let differentiator = Differentiator::try_new(1.0e-5)?;
    let interval = IntegrationInterval::try_new(0.0, 1.0)?;
    let integrator = Integrator::try_new(128)?;
    let limit = LimitApproximator::try_new(1.0e-6, 1.0e-5)?;

    let slope = differentiator.derivative_at(|x| x.powi(3), 2.0)?;
    let area = integrator.simpson(|x| x * x, interval)?;
    let sinc_limit = limit.at(
        |x| {
            if x == 0.0 { 1.0 } else { x.sin() / x }
        },
        0.0,
    )?;

    assert_close(slope, 12.0, 1.0e-3);
    assert_close(area, 1.0 / 3.0, 1.0e-6);
    assert_close(sinc_limit, 1.0, 1.0e-5);

    Ok(())
}

#[test]
fn symmetric_limit_reports_discontinuities_explicitly() {
    assert!(matches!(
        symmetric_limit(|x| if x < 0.0 { -1.0 } else { 1.0 }, 0.0, 1.0e-6, 1.0e-3,),
        Err(CalculusError::LimitMismatch { .. })
    ));
}
