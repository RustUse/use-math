use use_set::{
    are_disjoint, contains_member, is_subset, set_difference, set_intersection,
    set_symmetric_difference, set_union,
};

#[test]
fn direct_set_usage_covers_membership_and_operations() {
    let lhs = [1, 2, 2, 3];
    let rhs = [3, 4, 2, 5];

    assert!(contains_member(&lhs, &1));
    assert!(is_subset(&[2, 3], &rhs));
    assert!(!are_disjoint(&lhs, &rhs));

    assert_eq!(set_union(&lhs, &rhs), vec![1, 2, 3, 4, 5]);
    assert_eq!(set_intersection(&lhs, &rhs), vec![2, 3]);
    assert_eq!(set_difference(&lhs, &rhs), vec![1]);
    assert_eq!(set_symmetric_difference(&lhs, &rhs), vec![1, 4, 5]);
}
