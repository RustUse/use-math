use use_complex::{Complex, Imaginary};

fn main() {
    let value = Complex::new(3.0_f64, 4.0_f64);
    let mirrored = value.conjugate();
    let lifted = Complex::from(Imaginary::new(4.0_f64));
    let (magnitude, argument) = value.to_polar();

    assert_eq!(value + Complex::one(), Complex::new(4.0, 4.0));
    assert_eq!(mirrored, Complex::new(3.0, -4.0));
    assert_eq!(lifted, Complex::new(0.0, 4.0));
    assert!((magnitude - 5.0).abs() <= 1.0e-10);
    assert!((argument - value.argument()).abs() <= 1.0e-10);
}
