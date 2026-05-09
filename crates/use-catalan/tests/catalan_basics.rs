use use_catalan::{catalan, catalan_sequence, is_catalan_number};

#[test]
fn catalan_direct_api_covers_basic_values() {
    assert_eq!(catalan(0), Some(1));
    assert_eq!(catalan(1), Some(1));
    assert_eq!(catalan(5), Some(42));
    assert_eq!(catalan(10), Some(16796));
}

#[test]
fn catalan_sequence_direct_api() {
    assert_eq!(catalan_sequence(0), Some(vec![]));
    assert_eq!(catalan_sequence(6), Some(vec![1, 1, 2, 5, 14, 42]));
}

#[test]
fn is_catalan_number_direct_api() {
    assert!(is_catalan_number(1));
    assert!(is_catalan_number(42));
    assert!(!is_catalan_number(43));
}
