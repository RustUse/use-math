/// Commonly used facade items.
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
    IntegerError, IntegerSign, are_coprime, classify_sign, gcd, is_divisible_by, is_even,
    is_odd, lcm,
};

#[cfg(feature = "probability")]
pub use crate::{Bernoulli, Probability, ProbabilityError, independent_intersection, independent_union};

#[cfg(feature = "rational")]
pub use crate::{Rational, RationalError};

#[cfg(feature = "real")]
pub use crate::{Real, RealError, RealInterval, approx_eq};
