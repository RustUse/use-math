#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Geode-array primitives for `RustUse`.

pub mod error;
pub mod geode;
pub mod prelude;

pub use error::GeodeError;
pub use geode::{
    TypeVector, catalan_from_geode_dimension, checked_factorial, checked_product_factorials,
    diagonal_geode_2d, diagonal_geode_3d, diagonal_geode_4d, exact_divide, face_count, geode,
    geode_memoized, geode_on_first_axis, hyper_catalan, polygon_edge_count, polygon_vertex_count,
};
