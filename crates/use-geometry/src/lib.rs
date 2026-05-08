#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small Euclidean 2D geometry primitives for `RustUse`.

pub mod aabb;
pub mod circle;
pub mod distance;
pub mod error;
pub mod line;
pub mod orientation;
pub mod point;
pub mod prelude;
pub mod segment;
pub mod triangle;
pub mod vector;

pub use aabb::{Aabb2, aabb_from_points};
pub use circle::Circle;
pub use distance::{distance_2d, distance_squared_2d, midpoint_2d};
pub use error::GeometryError;
pub use line::Line2;
pub use orientation::{
    Orientation2, orientation_2d, orientation_2d_with_tolerance, try_orientation_2d,
    try_orientation_2d_with_tolerance,
};
pub use point::Point2;
pub use segment::Segment2;
pub use triangle::{Triangle, triangle_area, triangle_twice_area, triangle_twice_signed_area};
pub use vector::Vector2;
