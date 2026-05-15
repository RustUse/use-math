#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Utility-first facade for `RustUse` math crates.

#[cfg(feature = "arithmetic")]
pub use use_arithmetic as arithmetic;

#[cfg(feature = "arithmetic")]
pub use use_arithmetic::{
    CheckedArithmetic, SaturatingArithmetic, WrappingArithmetic, checked_add, checked_div_ceil,
    checked_div_floor, checked_is_divisible_by, checked_lcm, checked_mod_floor, checked_mul,
    checked_sub, div_ceil, div_floor, mod_floor, saturating_add, saturating_mul, saturating_sub,
    wrapping_add, wrapping_mul, wrapping_sub,
};

#[cfg(feature = "algebra")]
pub use use_algebra as algebra;

#[cfg(feature = "algebra")]
pub use use_algebra::{
    has_inverses, identity_element, is_abelian_group, is_associative, is_closed_under,
    is_commutative, is_distributive_over, is_group, is_monoid, is_ring,
};

#[cfg(feature = "calculus")]
pub use use_calculus as calculus;

#[cfg(feature = "calculus")]
pub use use_calculus::{
    CalculusError, Differentiator, IntegrationInterval, Integrator, LimitApproximator,
    central_difference, second_central_difference, simpson_integral, symmetric_limit,
    trapezoidal_integral,
};

#[cfg(feature = "catalan")]
pub use use_catalan as catalan;

#[cfg(feature = "catalan")]
pub use use_catalan::{CatalanError, catalan, fuss_catalan};

#[cfg(feature = "combinatorics")]
pub use use_combinatorics as combinatorics;

#[cfg(feature = "combinatorics")]
pub use use_combinatorics::{CombinatoricsError, combinations, factorial, permutations};

#[cfg(feature = "complex")]
pub use use_complex as complex;

#[cfg(feature = "complex")]
pub use use_complex::{Complex, Imaginary};

#[cfg(feature = "geometry")]
pub use use_geometry as geometry;

#[cfg(feature = "geometry")]
pub use use_geometry::{
    Aabb2, Circle, GeometryError, Line2, Orientation2, Point2, Segment2, Triangle, Vector2,
    aabb_from_points, distance_2d, distance_squared_2d, midpoint_2d, orientation_2d,
    orientation_2d_with_tolerance, triangle_area, triangle_twice_area, triangle_twice_signed_area,
    try_orientation_2d, try_orientation_2d_with_tolerance,
};

#[cfg(feature = "geode")]
pub mod geode {
    pub use use_geode::*;
}

#[cfg(feature = "geode")]
pub use use_geode::{
    GeodeError, TypeVector, catalan_from_geode_dimension, checked_factorial,
    checked_product_factorials, diagonal_geode_2d, diagonal_geode_3d, diagonal_geode_4d,
    exact_divide, face_count, geode_memoized, geode_on_first_axis, hyper_catalan,
    polygon_edge_count, polygon_vertex_count,
};

#[cfg(feature = "integer")]
pub use use_integer as integer;

#[cfg(feature = "integer")]
pub use use_integer::{
    IntegerError, IntegerSign, are_coprime, classify_sign, gcd, is_divisible_by, is_even, is_odd,
    lcm,
};

#[cfg(feature = "interval")]
pub use use_interval as interval;

#[cfg(feature = "interval")]
pub use use_interval::{Bound, Interval};

#[cfg(feature = "modular")]
pub use use_modular as modular;

#[cfg(feature = "modular")]
pub use use_modular::{
    Modular, is_congruent, mod_add, mod_inverse, mod_mul, mod_normalize, mod_pow, mod_sub,
};

#[cfg(feature = "prime")]
pub use use_prime as prime;

#[cfg(feature = "prime")]
pub use use_prime::{
    factorization, is_composite, is_prime, next_prime, previous_prime, prime_factors, primes_up_to,
    sieve, unique_prime_factors,
};

#[cfg(feature = "polynomial")]
pub use use_polynomial as polynomial;

#[cfg(feature = "polynomial")]
pub use use_polynomial::{Polynomial, linear_root, quadratic_roots};

#[cfg(feature = "equation")]
pub use use_equation as equation;

#[cfg(feature = "equation")]
pub use use_equation::{
    LinearEquation, LinearSystem2, QuadraticEquation, RootSolver, Roots, solve_linear,
    solve_quadratic,
};

#[cfg(all(feature = "equation", feature = "polynomial"))]
pub use use_equation::solve_polynomial_degree_1_or_2;

#[cfg(feature = "numerical")]
pub use use_numerical as numerical;

#[cfg(feature = "matrix")]
pub use use_matrix as matrix;

#[cfg(feature = "matrix")]
pub use use_matrix::{Matrix2, Matrix3, Matrix4};

#[cfg(feature = "linear")]
pub use use_linear as linear;

#[cfg(feature = "linear")]
pub use use_linear::{LinearError, solve_2x2};

#[cfg(feature = "vector")]
pub use use_vector as vector;

#[cfg(feature = "logic")]
pub use use_logic as logic;

#[cfg(feature = "logic")]
pub use use_logic::{equivalence, exclusive_or, implication, majority, nand, nor};

#[cfg(feature = "number")]
pub use use_number as number;

#[cfg(feature = "number")]
pub use use_number::{
    GOLDEN_RATIO, GOLDEN_RATIO_F32, NumberCategory, NumberSign, SQRT_3, SQRT_3_F32,
    classify_number, classify_number_sign, is_finite_number,
};

#[cfg(feature = "probability")]
pub use use_probability as probability;

#[cfg(feature = "probability")]
pub use use_probability::{
    Bernoulli, Probability, ProbabilityError, independent_intersection, independent_union,
};

#[cfg(feature = "rational")]
pub use use_rational as rational;

#[cfg(feature = "rational")]
pub use use_rational::{Rational, RationalError};

#[cfg(feature = "real")]
pub use use_real as real;

#[cfg(feature = "real")]
pub use use_real::{Real, RealError, RealInterval, approx_eq};

#[cfg(feature = "series")]
pub use use_series as series;

#[cfg(feature = "series")]
pub use use_series::{
    SeriesError, arithmetic_nth_term, arithmetic_sum, geometric_nth_term, geometric_sum,
};

#[cfg(feature = "set")]
pub use use_set as set;

#[cfg(feature = "set")]
pub use use_set::{
    are_disjoint, contains_member, is_subset, set_difference, set_intersection,
    set_symmetric_difference, set_union,
};

#[cfg(feature = "statistics")]
pub use use_statistics as statistics;

#[cfg(feature = "statistics")]
pub use use_statistics::{
    StatisticsError, mean, median, population_std_dev, population_variance, sample_std_dev,
    sample_variance,
};

#[cfg(feature = "trigonometry")]
pub use use_trigonometry as trigonometry;

#[cfg(feature = "trigonometry")]
pub use use_trigonometry::{
    Angle, cos_deg, degrees_to_radians, normalize_degrees, normalize_radians, radians_to_degrees,
    sin_deg, tan_deg,
};

pub mod prelude;
