# use-geometry

<p align="center">
  <strong>Small Euclidean 2D geometry primitives for direct, predictable Rust code.</strong><br>
  Validated constructors, tolerance-aware helpers, and composable value types without a full geometry engine.
</p>

<p align="center">
  <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
  <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
  <img alt="2D Euclidean API" src="https://img.shields.io/badge/geometry-2D%20Euclidean-1d4ed8">
  <img alt="Validated constructors" src="https://img.shields.io/badge/validation-try__new-c2410c">
  <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

<p align="center">
  <a href="#what-this-crate-provides">Surface</a> ·
  <a href="#when-to-use-it-directly">When to use it</a> ·
  <a href="#installation">Installation</a> ·
  <a href="#quick-examples">Examples</a> ·
  <a href="#validation-model">Validation</a> ·
  <a href="#scope">Scope</a>
</p>

`use-geometry` provides small, composable geometry building blocks rather than a large geometry engine. The crate keeps the model explicit: direct structs, standalone helpers, validated constructors for external input, and tolerance-aware APIs only where ambiguity needs to be made explicit.

<table>
  <tr>
    <td width="33%" valign="top">
      <strong>Foundation types</strong><br>
      <code>Point2</code>, <code>Vector2</code>, and measurement helpers for direct coordinate math.
    </td>
    <td width="33%" valign="top">
      <strong>Shapes and bounds</strong><br>
      <code>Line2</code>, <code>Segment2</code>, <code>Circle</code>, <code>Triangle</code>, and <code>Aabb2</code>.
    </td>
    <td width="33%" valign="top">
      <strong>Explicit validation</strong><br>
      Use <code>try_new</code> and tolerance-aware helpers when inputs come from users, files, or networks.
    </td>
  </tr>
</table>

## What this crate provides

| Area                    | Root exports                                                                     | Best fit                                      |
| ----------------------- | -------------------------------------------------------------------------------- | --------------------------------------------- |
| Coordinates and vectors | `Point2`, `Vector2`                                                              | Direct 2D value math                          |
| Distance and midpoint   | `distance_2d`, `distance_squared_2d`, `midpoint_2d`                              | Common point-to-point calculations            |
| Orientation             | `Orientation2`, `orientation_2d`, `try_orientation_2d`, tolerance-aware variants | Winding and collinearity checks               |
| Lines and segments      | `Line2`, `Segment2`                                                              | Directed 2D relationships and interpolation   |
| Shapes and bounds       | `Circle`, `Triangle`, `Aabb2`, `aabb_from_points`                                | Shape metrics, containment, and bounds checks |

| If you need to...                                      | Start here                                      |
| ------------------------------------------------------ | ----------------------------------------------- |
| Validate coordinates and shapes from external input    | `try_new` constructors                          |
| Work with trusted values in hot paths                  | Infallible constructors like `Point2::new(...)` |
| Build simple containment or broad-phase checks         | `Aabb2`                                         |
| Choose exact vs approximate geometric tests explicitly | Tolerance-aware helpers                         |

## When to use it directly

Choose `use-geometry` directly when geometry is the only math surface your application needs, or when you want to avoid pulling a broader facade into the dependency graph.

| Scenario                                                         | Use `use-geometry` directly? | Why                                                    |
| ---------------------------------------------------------------- | ---------------------------- | ------------------------------------------------------ |
| You only need 2D primitives and measurements                     | Yes                          | The crate stays narrow and explicit                    |
| You want validated construction at the boundary of your app      | Yes                          | `try_new` and tolerance helpers make failures explicit |
| You also need checked combinatorics                              | Usually no                   | `use-math` may be the better integration point         |
| You need a broad geometry engine with intersections and polygons | No                           | This crate intentionally stops short of that scope     |

## Installation

```toml
[dependencies]
use-geometry = "0.0.1"
```

## Quick examples

### Validated construction from external input

```rust
use use_geometry::{
    Aabb2, Circle, Orientation2, Point2, Segment2, Triangle, midpoint_2d, try_orientation_2d,
};

let a = Point2::try_new(0.0, 0.0)?;
let b = Point2::try_new(4.0, 0.0)?;
let c = Point2::try_new(0.0, 3.0)?;

let segment = Segment2::try_new(a, b)?;
let triangle = Triangle::try_new(a, b, c)?;
let circle = Circle::try_new(a, 3.0)?;
let bounds = Aabb2::from_points(a, c);

assert_eq!(segment.midpoint(), midpoint_2d(a, b));
assert_eq!(triangle.area(), 6.0);
assert_eq!(triangle.perimeter(), 12.0);
assert_eq!(try_orientation_2d(a, b, c)?, Orientation2::CounterClockwise);
assert_eq!(circle.center(), a);
assert_eq!(circle.radius(), 3.0);
assert!(bounds.contains_point(Point2::new(0.0, 1.5)));
# Ok::<(), use_geometry::GeometryError>(())
```

### Bounds and tolerance-aware orientation

```rust
use use_geometry::{Aabb2, Orientation2, Point2, orientation_2d_with_tolerance};

let bounds = Aabb2::from_points(Point2::new(4.0, 1.0), Point2::new(1.0, 3.0));

assert_eq!(bounds.min(), Point2::new(1.0, 1.0));
assert_eq!(bounds.max(), Point2::new(4.0, 3.0));
assert_eq!(bounds.center(), Point2::new(2.5, 2.0));
assert!(bounds.contains_point(Point2::new(2.0, 2.0)));
assert_eq!(
    orientation_2d_with_tolerance(
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(2.0, 2.0 + 1.0e-12),
        1.0e-11,
    )?,
    Orientation2::Collinear,
);
# Ok::<(), use_geometry::GeometryError>(())
```

### Shape metrics stay direct

```rust
use use_geometry::{Orientation2, Point2, Triangle, triangle_area, triangle_twice_signed_area};

let a = Point2::try_new(0.0, 0.0)?;
let b = Point2::try_new(4.0, 0.0)?;
let c = Point2::try_new(0.0, 3.0)?;
let triangle = Triangle::try_new(a, b, c)?;

assert_eq!(triangle.orientation(), Orientation2::CounterClockwise);
assert_eq!(triangle.twice_signed_area(), triangle_twice_signed_area(a, b, c));
assert_eq!(triangle.area(), triangle_area(a, b, c));
assert_eq!(triangle.sides(), [4.0, 5.0, 3.0]);
assert_eq!(triangle.perimeter(), 12.0);
assert_eq!(triangle.centroid(), Point2::new(4.0 / 3.0, 1.0));
assert!(!triangle.is_degenerate());
# Ok::<(), use_geometry::GeometryError>(())
```

## Degeneracy stays explicit

> [!IMPORTANT]
> Exact degeneracy and tolerance-based degeneracy are different checks. In `use-geometry`, callers should choose deliberately based on coordinate scale, input quality, and whether they care about mathematical collapse or practical numeric collapse.

Degeneracy occurs when a geometric value collapses into a lower-dimensional or invalid form. For triangles, exact degeneracy means the vertices are collinear and the signed twice-area is exactly zero.

`f64::EPSILON` is usually not a useful geometry tolerance. It is tied to floating-point precision around `1.0`, not the coordinate scale of a triangle. Practical degeneracy checks should use a caller-provided tolerance chosen for the coordinate scale, input source, and expected numeric noise.

```rust
use use_geometry::{Point2, Triangle};

let triangle = Triangle::try_new(
  Point2::try_new(0.0, 0.0)?,
  Point2::try_new(4.0, 0.0)?,
  Point2::try_new(8.0, 1.0e-13)?,
)?;

assert!(!triangle.is_degenerate());
assert!(triangle.is_degenerate_with_tolerance(1.0e-12)?);
# Ok::<(), use_geometry::GeometryError>(())
```

## Validation model

Use `try_new` when values may come from user input, files, network payloads, or other untrusted sources. Use infallible constructors like `Point2::new(...)`, `Vector2::new(...)`, `Triangle::new(...)`, or `Aabb2::from_points(...)` when values are already trusted and you want direct assembly.

> [!IMPORTANT]
> Tolerance-aware helpers reject negative and non-finite tolerances explicitly. This crate does not hide geometric ambiguity behind a global epsilon policy.

## Scope

- Small APIs are preferred over broad trait-heavy abstractions.
- Primitives are intended to compose cleanly with each other and with plain `f64` values.
- Built-in approximate equality policies and global epsilon choices are intentionally out of scope.
- Polygon, intersection, and spatial indexing APIs are intentionally deferred.
- Plane-oriented APIs are deferred until a later 3D geometry layer.

## Status

`use-geometry` is a scaffolded public crate in the `RustUse` docs surface. The API remains pre-1.0, and the `RustUse`-hosted generated rustdocs stay canonical while external crates.io and docs.rs pages remain staged.
