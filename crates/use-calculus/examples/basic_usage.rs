use use_calculus::{Differentiator, IntegrationInterval, Integrator, LimitApproximator};

fn main() -> Result<(), use_calculus::CalculusError> {
    let differentiator = Differentiator::try_new(1.0e-5)?;
    let interval = IntegrationInterval::try_new(0.0, 1.0)?;
    let integrator = Integrator::try_new(128)?;
    let limit = LimitApproximator::try_new(1.0e-6, 1.0e-5)?;

    let slope = differentiator.derivative_at(|x| x.powi(2), 3.0)?;
    let area = integrator.simpson(|x| x * x, interval)?;
    let sinc_limit = limit.at(
        |x| {
            if x == 0.0 { 1.0 } else { x.sin() / x }
        },
        0.0,
    )?;

    assert!((slope - 6.0).abs() < 1.0e-6);
    assert!((area - (1.0 / 3.0)).abs() < 1.0e-6);
    assert!((sinc_limit - 1.0).abs() < 1.0e-5);

    Ok(())
}
