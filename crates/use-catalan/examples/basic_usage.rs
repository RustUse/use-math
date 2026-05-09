use use_catalan::{catalan, fuss_catalan};

fn main() -> Result<(), use_catalan::CatalanError> {
    assert_eq!(catalan(4)?, 14);
    assert_eq!(fuss_catalan(3, 3)?, 12);

    Ok(())
}
