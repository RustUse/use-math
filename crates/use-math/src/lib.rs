#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Utility-first facade for `RustUse` math crates.

#[cfg(feature = "combinatorics")]
pub use use_combinatorics as combinatorics;

#[cfg(feature = "combinatorics")]
pub use use_combinatorics::{CombinatoricsError, combinations, factorial, permutations};

#[cfg(feature = "geometry")]
pub use use_geometry as geometry;

#[cfg(feature = "geometry")]
pub use use_geometry::{
    Aabb2, Circle, GeometryError, Line2, Orientation2, Point2, Segment2, Triangle, Vector2,
    aabb_from_points, distance_2d, distance_squared_2d, midpoint_2d, orientation_2d,
    orientation_2d_with_tolerance, triangle_area, triangle_twice_area, triangle_twice_signed_area,
    try_orientation_2d, try_orientation_2d_with_tolerance,
};

#[cfg(feature = "series")]
pub use use_series as series;

#[cfg(feature = "series")]
pub use use_series::Series;

pub mod prelude;
