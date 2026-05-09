use use_algebra::{
    has_inverses, identity_element, is_abelian_group, is_associative, is_closed_under,
    is_commutative, is_distributive_over, is_group, is_monoid, is_ring,
};

#[test]
fn direct_algebra_usage_covers_group_and_ring_checks() {
    let residues = [0_u8, 1, 2];
    let add_mod_3 = |left, right| (left + right) % 3;
    let mul_mod_3 = |left, right| (left * right) % 3;

    assert!(is_closed_under(&residues, add_mod_3));
    assert!(is_associative(&residues, add_mod_3));
    assert!(is_commutative(&residues, add_mod_3));
    assert_eq!(identity_element(&residues, add_mod_3), Some(0));
    assert!(has_inverses(&residues, add_mod_3, 0));
    assert!(is_monoid(&residues, add_mod_3));
    assert!(is_group(&residues, add_mod_3));
    assert!(is_abelian_group(&residues, add_mod_3));
    assert!(is_distributive_over(&residues, mul_mod_3, add_mod_3));
    assert!(is_ring(&residues, add_mod_3, mul_mod_3));
}

#[test]
fn algebra_usage_rejects_structures_that_fail_the_laws() {
    let booleans = [false, true];
    let and = |left, right| left && right;
    let values = [0_u8, 1];
    let add = |left, right| left + right;

    assert_eq!(identity_element(&booleans, and), Some(true));
    assert!(is_monoid(&booleans, and));
    assert!(!is_group(&booleans, and));
    assert!(!is_closed_under(&values, add));
    assert!(!is_ring(&values, add, add));
}
