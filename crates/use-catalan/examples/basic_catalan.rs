use use_catalan::{CatalanSequence, catalan, catalan_sequence, is_catalan_number};

fn main() {
    let first_six = catalan_sequence(6).unwrap_or_default();
    println!("First 6 Catalan numbers: {first_six:?}");

    let c5 = catalan(5).unwrap_or(0);
    println!("C(5) = {c5}");

    println!("is_catalan_number(42) = {}", is_catalan_number(42));
    println!("is_catalan_number(43) = {}", is_catalan_number(43));

    println!("Iterator (first 11):");
    for (n, c) in CatalanSequence::new().take(11).enumerate() {
        println!("  C({n}) = {c}");
    }
}
