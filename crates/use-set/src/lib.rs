#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Set utilities for `RustUse`.

/// Returns whether `value` is a member of `set`.
#[must_use]
pub fn contains_member<T>(set: &[T], value: &T) -> bool
where
    T: PartialEq,
{
    set.contains(value)
}

/// Returns whether every unique value in `left` appears in `right`.
#[must_use]
pub fn is_subset<T>(left: &[T], right: &[T]) -> bool
where
    T: PartialEq,
{
    left.iter().all(|value| right.contains(value))
}

/// Returns whether `left` and `right` share no common members.
#[must_use]
pub fn are_disjoint<T>(left: &[T], right: &[T]) -> bool
where
    T: PartialEq,
{
    left.iter().all(|value| !right.contains(value))
}

/// Returns the unique members of `left ∪ right` in first-seen order.
#[must_use]
pub fn set_union<T>(left: &[T], right: &[T]) -> Vec<T>
where
    T: Clone + PartialEq,
{
    let mut values = Vec::new();

    append_unique(&mut values, left);
    append_unique(&mut values, right);

    values
}

/// Returns the unique members of `left ∩ right` in the order they first appear in `left`.
#[must_use]
pub fn set_intersection<T>(left: &[T], right: &[T]) -> Vec<T>
where
    T: Clone + PartialEq,
{
    let mut values = Vec::new();

    for value in left {
        if right.contains(value) {
            push_unique(&mut values, value);
        }
    }

    values
}

/// Returns the unique members of `left \ right` in the order they first appear in `left`.
#[must_use]
pub fn set_difference<T>(left: &[T], right: &[T]) -> Vec<T>
where
    T: Clone + PartialEq,
{
    let mut values = Vec::new();

    for value in left {
        if !right.contains(value) {
            push_unique(&mut values, value);
        }
    }

    values
}

/// Returns the unique members that appear in exactly one of `left` or `right`.
#[must_use]
pub fn set_symmetric_difference<T>(left: &[T], right: &[T]) -> Vec<T>
where
    T: Clone + PartialEq,
{
    let mut values = set_difference(left, right);

    for value in right {
        if !left.contains(value) {
            push_unique(&mut values, value);
        }
    }

    values
}

fn append_unique<T>(target: &mut Vec<T>, source: &[T])
where
    T: Clone + PartialEq,
{
    for value in source {
        push_unique(target, value);
    }
}

fn push_unique<T>(target: &mut Vec<T>, value: &T)
where
    T: Clone + PartialEq,
{
    if !target.contains(value) {
        target.push(value.clone());
    }
}

pub mod prelude;

#[cfg(test)]
mod tests {
    use super::{
        are_disjoint, contains_member, is_subset, set_difference, set_intersection,
        set_symmetric_difference, set_union,
    };

    #[test]
    fn evaluates_membership_and_relations() {
        let left = [1, 2, 2, 3];
        let right = [2, 3, 4];
        let subset = [2, 2, 3];

        assert!(contains_member(&left, &2));
        assert!(!contains_member(&left, &5));
        assert!(is_subset(&subset, &right));
        assert!(!is_subset(&left, &right));
        assert!(are_disjoint(&[7, 8], &right));
        assert!(!are_disjoint(&left, &right));
    }

    #[test]
    fn computes_order_preserving_set_operations() {
        let left = [1, 2, 2, 3];
        let right = [3, 4, 2, 5];

        assert_eq!(set_union(&left, &right), vec![1, 2, 3, 4, 5]);
        assert_eq!(set_intersection(&left, &right), vec![2, 3]);
        assert_eq!(set_difference(&left, &right), vec![1]);
        assert_eq!(set_symmetric_difference(&left, &right), vec![1, 4, 5]);
    }
}
