#[cfg(feature = "equation")]
#[test]
fn facade_supports_equation_namespace_and_root_reexports() {
    use use_math::{
        LinearEquation, LinearSystem2, QuadraticEquation, RootSolver, Roots, equation,
        solve_linear, solve_quadratic,
    };

    assert_eq!(solve_linear(2.0, -4.0), Roots::One(2.0));
    assert_eq!(solve_quadratic(1.0, -3.0, 2.0), Roots::Two(1.0, 2.0));
    assert_eq!(
        equation::solve_2x2(2.0, 1.0, 5.0, 1.0, -1.0, 1.0),
        Some((2.0, 1.0))
    );
    assert_eq!(LinearEquation::new(2.0, -4.0).solve(), Roots::One(2.0));
    assert_eq!(
        QuadraticEquation::new(1.0, -2.0, 1.0).solve(),
        Roots::One(1.0)
    );
    assert_eq!(
        LinearSystem2::new(2.0, 1.0, 5.0, 1.0, -1.0, 1.0).solve(),
        Some((2.0, 1.0))
    );
}

#[cfg(all(feature = "equation", feature = "polynomial"))]
#[test]
fn facade_supports_equation_polynomial_bridge() {
    use use_math::{Roots, equation, solve_polynomial_degree_1_or_2};

    let polynomial = use_math::polynomial::Polynomial::new(vec![2.0, -3.0, 1.0]);

    assert_eq!(
        solve_polynomial_degree_1_or_2(&polynomial),
        Some(Roots::Two(1.0, 2.0))
    );
    assert_eq!(
        equation::solve_polynomial_degree_1_or_2(&polynomial),
        Some(Roots::Two(1.0, 2.0))
    );
}
