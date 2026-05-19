/// Commonly used facade items.
#[cfg(feature = "arithmetic")]
pub use crate::{
    checked_add, checked_div_ceil, checked_div_floor, checked_is_divisible_by, checked_lcm,
    checked_mod_floor, checked_mul, checked_sub, div_ceil, div_floor, mod_floor, saturating_add,
    saturating_mul, saturating_sub, wrapping_add, wrapping_mul, wrapping_sub,
};

#[cfg(feature = "algebra")]
pub use crate::{
    has_inverses, identity_element, is_abelian_group, is_associative, is_closed_under,
    is_commutative, is_distributive_over, is_group, is_monoid, is_ring,
};

#[cfg(feature = "catalan")]
pub use crate::{CatalanError, catalan, fuss_catalan};

#[cfg(feature = "collatz")]
pub use crate::{
    CollatzParity, CollatzRangeSummary, collatz_next, collatz_sequence, max_value_in_trajectory,
    parity, parity_vector, reaches_one, stopping_time, total_stopping_time, trajectory_len,
    verify_range,
};

#[cfg(feature = "calculus")]
pub use crate::{
    CalculusError, Differentiator, IntegrationInterval, Integrator, LimitApproximator,
    central_difference, second_central_difference, simpson_integral, symmetric_limit,
    trapezoidal_integral,
};

#[cfg(feature = "combinatorics")]
pub use crate::{CombinatoricsError, combinations, factorial, permutations};

#[cfg(feature = "complex")]
pub use crate::{Complex, Imaginary};

#[cfg(feature = "geode")]
pub use crate::{
    GeodeError, TypeVector, face_count, geode_memoized, hyper_catalan, polygon_edge_count,
    polygon_vertex_count,
};

#[cfg(feature = "geode")]
pub use crate::geode::geode;

#[cfg(feature = "integer")]
pub use crate::{
    IntegerError, IntegerSign, are_coprime, classify_sign, gcd, is_divisible_by, is_even, is_odd,
    lcm,
};

#[cfg(feature = "interval")]
pub use crate::{Bound, Interval};

#[cfg(feature = "modular")]
pub use crate::{
    Modular, is_congruent, mod_add, mod_inverse, mod_mul, mod_normalize, mod_pow, mod_sub,
};

#[cfg(feature = "prime")]
pub use crate::{
    factorization, is_composite, is_prime, next_prime, previous_prime, prime_factors, primes_up_to,
    sieve, unique_prime_factors,
};

#[cfg(feature = "polynomial")]
pub use crate::{Polynomial, linear_root, quadratic_roots};

#[cfg(feature = "equation")]
pub use crate::{
    LinearEquation, LinearSystem2, QuadraticEquation, RootSolver, Roots, solve_linear,
    solve_quadratic,
};

#[cfg(all(feature = "equation", feature = "polynomial"))]
pub use crate::solve_polynomial_degree_1_or_2;

#[cfg(feature = "eigen")]
pub use crate::{
    EigenError, EigenMultiplicity, EigenSpace, Eigenpair, Eigensystem, Eigenvalue, Eigenvector,
};

#[cfg(feature = "logic")]
pub use crate::{equivalence, exclusive_or, implication, majority, nand, nor};

#[cfg(feature = "matrix")]
pub use crate::{Matrix2, Matrix3, Matrix4};

#[cfg(feature = "linear")]
pub use crate::{LinearError, solve_2x2};

#[cfg(feature = "vector")]
pub use crate::{Vector, Vector2, Vector3, Vector4};

#[cfg(feature = "number")]
pub use crate::{
    GOLDEN_RATIO, GOLDEN_RATIO_F32, NumberCategory, NumberSign, SQRT_3, SQRT_3_F32,
    classify_number, classify_number_sign, is_finite_number,
};

#[cfg(feature = "probability")]
pub use crate::{
    Bernoulli, Probability, ProbabilityError, independent_intersection, independent_union,
};

#[cfg(feature = "rational")]
pub use crate::{Rational, RationalError};

#[cfg(feature = "real")]
pub use crate::{Real, RealError, RealInterval, approx_eq};

#[cfg(feature = "series")]
pub use crate::{
    SeriesError, arithmetic_nth_term, arithmetic_sum, geometric_nth_term, geometric_sum,
};

#[cfg(feature = "set")]
pub use crate::{
    are_disjoint, contains_member, is_subset, set_difference, set_intersection,
    set_symmetric_difference, set_union,
};

#[cfg(feature = "statistics")]
pub use crate::{
    StatisticsError, mean, median, population_std_dev, population_variance, sample_std_dev,
    sample_variance,
};

#[cfg(feature = "trigonometry")]
pub use crate::{
    Angle, cos_deg, degrees_to_radians, normalize_degrees, normalize_radians, radians_to_degrees,
    sin_deg, tan_deg,
};
