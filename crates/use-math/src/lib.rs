#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Utility-first facade for `RustUse` math crates.

#[cfg(feature = "algebra")]
pub use use_algebra as algebra;

#[cfg(feature = "calculus")]
pub use use_calculus as calculus;

#[cfg(feature = "catalan")]
pub use use_catalan as catalan;

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

#[cfg(feature = "integer")]
pub use use_integer as integer;

#[cfg(feature = "linear")]
pub use use_linear as linear;

#[cfg(feature = "logic")]
pub use use_logic as logic;

#[cfg(feature = "number")]
pub use use_number as number;

#[cfg(feature = "probability")]
pub use use_probability as probability;

#[cfg(feature = "rational")]
pub use use_rational as rational;

#[cfg(feature = "real")]
pub use use_real as real;

#[cfg(feature = "series")]
pub use use_series as series;

#[cfg(feature = "set")]
pub use use_set as set;

#[cfg(feature = "statistics")]
pub use use_statistics as statistics;

#[cfg(feature = "trigonometry")]
pub use use_trigonometry as trigonometry;

pub mod prelude;
