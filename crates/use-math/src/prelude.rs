/// Commonly used facade items.
#[cfg(feature = "algebra")]
pub use crate::{
    has_inverses, identity_element, is_abelian_group, is_associative, is_closed_under,
    is_commutative, is_distributive_over, is_group, is_monoid, is_ring,
};

#[cfg(feature = "catalan")]
pub use crate::{CatalanError, catalan, fuss_catalan};

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

#[cfg(feature = "geometry")]
pub use crate::{
    Aabb2, Circle, GeometryError, Line2, Orientation2, Point2, Segment2, Triangle, Vector2,
    aabb_from_points, distance_2d, distance_squared_2d, midpoint_2d, orientation_2d,
    orientation_2d_with_tolerance, triangle_area, triangle_twice_area, triangle_twice_signed_area,
    try_orientation_2d, try_orientation_2d_with_tolerance,
};

#[cfg(feature = "integer")]
pub use crate::{
    IntegerError, IntegerSign, are_coprime, classify_sign, gcd, is_divisible_by, is_even, is_odd,
    lcm,
};

#[cfg(feature = "logic")]
pub use crate::{equivalence, exclusive_or, implication, majority, nand, nor};

#[cfg(feature = "linear")]
pub use crate::{LinearError, LinearVector2, Matrix2, dot, solve_2x2};

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
