/// Commonly used facade items.
#[cfg(feature = "combinatorics")]
pub use crate::{CombinatoricsError, combinations, factorial, permutations};

#[cfg(feature = "geometry")]
pub use crate::{
    Aabb2, Circle, GeometryError, Line2, Orientation2, Point2, Segment2, Triangle, Vector2,
    aabb_from_points, distance_2d, distance_squared_2d, midpoint_2d, orientation_2d,
    orientation_2d_with_tolerance, triangle_area, triangle_twice_area, triangle_twice_signed_area,
    try_orientation_2d, try_orientation_2d_with_tolerance,
};
