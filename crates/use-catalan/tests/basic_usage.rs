use use_catalan::{CatalanError, catalan, fuss_catalan};

#[test]
fn direct_catalan_usage_covers_core_counting() -> Result<(), CatalanError> {
    assert_eq!(catalan(0)?, 1);
    assert_eq!(catalan(4)?, 14);
    assert_eq!(catalan(10)?, 16_796);
    assert_eq!(fuss_catalan(2, 4)?, 14);
    assert_eq!(fuss_catalan(3, 3)?, 12);

    Ok(())
}

#[test]
fn catalan_validation_rejects_invalid_orders_and_reports_overflow() {
    assert_eq!(fuss_catalan(0, 3), Err(CatalanError::ZeroOrder));
    assert!(matches!(
        catalan(100),
        Err(CatalanError::CatalanOverflow(100))
    ));
    assert!(matches!(
        fuss_catalan(5, 60),
        Err(CatalanError::FussCatalanOverflow { order: 5, n: 60 })
    ));
}
