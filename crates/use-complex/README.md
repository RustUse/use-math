# use-complex

<p align="center">
    <strong>Composable complex-number and imaginary-number primitives for `RustUse`.</strong><br>
    Small, explicit rectangular and polar helpers without pulling in a large numeric framework.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Status primitives" src="https://img.shields.io/badge/status-primitives-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

`use-complex` provides a small complex-number layer for `RustUse`: rectangular-form storage, an explicit `Imaginary<T>` newtype, standard arithmetic operators, and float-only polar helpers where the standard library already provides the necessary trigonometric functions.

## Install

```toml
[dependencies]
use-complex = "0.0.1"
```

## Basic usage

```rust
use use_complex::Complex;

let value = Complex::new(3.0_f64, 4.0_f64);

assert_eq!(value.real(), &3.0);
assert_eq!(value.imaginary(), &4.0);
assert_eq!(value.magnitude_squared(), 25.0);
assert_eq!(value.to_string(), "3 + 4i");
```

## Arithmetic

```rust
use use_complex::Complex;

let lhs = Complex::new(3.0_f64, 4.0_f64);
let rhs = Complex::new(1.0_f64, -2.0_f64);

assert_eq!(lhs + rhs, Complex::new(4.0, 2.0));
assert_eq!(lhs - rhs, Complex::new(2.0, 6.0));
assert_eq!(lhs * rhs, Complex::new(11.0, -2.0));

let quotient = lhs / rhs;
assert!((quotient.re + 1.0).abs() <= 1.0e-10);
assert!((quotient.im - 2.0).abs() <= 1.0e-10);
```

## Imaginary numbers

```rust
use use_complex::{Complex, Imaginary};

let imaginary = Imaginary::new(4_i32);
let lifted = Complex::from(imaginary);

assert_eq!(imaginary.to_string(), "4i");
assert_eq!(lifted, Complex::new(0, 4));
```

## Polar form

```rust
use core::f64::consts::FRAC_PI_3;
use use_complex::Complex;

let value = Complex::<f64>::from_polar(5.0_f64, FRAC_PI_3);
let (magnitude, argument) = value.to_polar();

assert!((magnitude - 5.0).abs() <= 1.0e-10);
assert!((argument - FRAC_PI_3).abs() <= 1.0e-10);
```

## License

Licensed under MIT OR Apache-2.0.