# use-interval

<p align="center">
    <strong>Small interval and bound primitives for <code>RustUse</code>.</strong><br>
    Explicit open, closed, half-open, and unbounded intervals with containment, overlap, and intersection operations.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Interval primitives" src="https://img.shields.io/badge/interval-primitives-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-interval = "0.0.5"
```

## What belongs here

`use-interval` owns reusable interval and bound primitives. It provides open,
closed, half-open, and unbounded intervals together with small operations such
as containment, overlap, emptiness checks, and intersection.

The crate stays intentionally narrow and dependency-free so adjacent crates can
reuse interval semantics without pulling in domain-specific policy.

## Neighboring crates

| Crate              | Responsibility                                                     |
| ------------------ | ------------------------------------------------------------------ |
| `use-interval`     | Generic interval and bound primitives                              |
| `use-real`         | Real-number abstractions, validated finite values, and comparisons |
| `use-calculus`     | Numerical methods that may consume intervals                       |
| `use-probability`  | Probability-specific interval interpretations                      |
| `use-statistics`   | Statistical confidence intervals and descriptive interpretation    |
| `use-geometry`     | Geometry-specific bounds, boxes, and spatial interval compositions |
| `use-optimization` | Optimization algorithms and interval-based search strategies       |

`use-interval` intentionally does not define confidence interval types,
geometry-specific bounding boxes, unit-aware intervals, or equation-solving
algorithms.

## Examples

### Containment

```rust
use use_interval::Interval;

let interval = Interval::closed(1.0, 3.0);

assert!(interval.contains(1.0));
assert!(interval.contains(2.0));
assert!(interval.contains(3.0));
assert!(!interval.contains(4.0));
```

### Overlap and excluded endpoints

```rust
use use_interval::Interval;

let left = Interval::closed(1.0, 3.0);
let right = Interval::open(3.0, 5.0);

assert!(!left.overlaps(&right));
assert_eq!(left.intersection(&right), None);
```

### Intersection with an included shared endpoint

```rust
use use_interval::Interval;

let left = Interval::closed(1.0, 3.0);
let right = Interval::closed(3.0, 5.0);

assert!(left.overlaps(&right));
assert_eq!(left.intersection(&right), Some(Interval::closed(3.0, 3.0)));
```

## Status

`use-interval` is a concrete pre-1.0 crate in the `RustUse` math workspace. The
API stays explicit and conservative so crates like `use-real`,
`use-calculus`, `use-probability`, and `use-optimization` can share one small
interval vocabulary without inheriting domain-specific abstractions.
