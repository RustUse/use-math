//! Finite-sample algebraic law helpers.

/// Returns whether applying `operation` to any pair of values in `elements`
/// yields another member of `elements`.
#[must_use]
pub fn is_closed_under<T, F>(elements: &[T], operation: F) -> bool
where
    T: Copy + PartialEq,
    F: Fn(T, T) -> T + Copy,
{
    for left in elements.iter().copied() {
        for right in elements.iter().copied() {
            let result = operation(left, right);

            if !elements.contains(&result) {
                return false;
            }
        }
    }

    true
}

/// Returns whether `operation` is associative over `elements`.
#[must_use]
pub fn is_associative<T, F>(elements: &[T], operation: F) -> bool
where
    T: Copy + PartialEq,
    F: Fn(T, T) -> T + Copy,
{
    for first in elements.iter().copied() {
        for second in elements.iter().copied() {
            for third in elements.iter().copied() {
                if operation(operation(first, second), third)
                    != operation(first, operation(second, third))
                {
                    return false;
                }
            }
        }
    }

    true
}

/// Returns whether `operation` is commutative over `elements`.
#[must_use]
pub fn is_commutative<T, F>(elements: &[T], operation: F) -> bool
where
    T: Copy + PartialEq,
    F: Fn(T, T) -> T + Copy,
{
    for left in elements.iter().copied() {
        for right in elements.iter().copied() {
            if operation(left, right) != operation(right, left) {
                return false;
            }
        }
    }

    true
}

/// Returns a two-sided identity element for `operation`, if one exists in
/// `elements`.
#[must_use]
pub fn identity_element<T, F>(elements: &[T], operation: F) -> Option<T>
where
    T: Copy + PartialEq,
    F: Fn(T, T) -> T + Copy,
{
    'candidate: for candidate in elements.iter().copied() {
        for value in elements.iter().copied() {
            if operation(candidate, value) != value || operation(value, candidate) != value {
                continue 'candidate;
            }
        }

        return Some(candidate);
    }

    None
}

/// Returns whether every value in `elements` has a two-sided inverse with
/// respect to `identity` and `operation`.
#[must_use]
pub fn has_inverses<T, F>(elements: &[T], operation: F, identity: T) -> bool
where
    T: Copy + PartialEq,
    F: Fn(T, T) -> T + Copy,
{
    if !elements.contains(&identity) {
        return false;
    }

    for value in elements.iter().copied() {
        let mut found_inverse = false;

        for candidate in elements.iter().copied() {
            if operation(value, candidate) == identity && operation(candidate, value) == identity {
                found_inverse = true;
                break;
            }
        }

        if !found_inverse {
            return false;
        }
    }

    true
}

/// Returns whether `elements` with `operation` form a monoid.
#[must_use]
pub fn is_monoid<T, F>(elements: &[T], operation: F) -> bool
where
    T: Copy + PartialEq,
    F: Fn(T, T) -> T + Copy,
{
    is_closed_under(elements, operation)
        && is_associative(elements, operation)
        && identity_element(elements, operation).is_some()
}

/// Returns whether `elements` with `operation` form a group.
#[must_use]
pub fn is_group<T, F>(elements: &[T], operation: F) -> bool
where
    T: Copy + PartialEq,
    F: Fn(T, T) -> T + Copy,
{
    let Some(identity) = identity_element(elements, operation) else {
        return false;
    };

    is_closed_under(elements, operation)
        && is_associative(elements, operation)
        && has_inverses(elements, operation, identity)
}

/// Returns whether `elements` with `operation` form an abelian group.
#[must_use]
pub fn is_abelian_group<T, F>(elements: &[T], operation: F) -> bool
where
    T: Copy + PartialEq,
    F: Fn(T, T) -> T + Copy,
{
    is_group(elements, operation) && is_commutative(elements, operation)
}

/// Returns whether `multiply` distributes over `add` from both sides on
/// `elements`.
#[must_use]
pub fn is_distributive_over<T, Mul, Add>(elements: &[T], multiply: Mul, add: Add) -> bool
where
    T: Copy + PartialEq,
    Mul: Fn(T, T) -> T + Copy,
    Add: Fn(T, T) -> T + Copy,
{
    for left in elements.iter().copied() {
        for middle in elements.iter().copied() {
            for right in elements.iter().copied() {
                if multiply(left, add(middle, right))
                    != add(multiply(left, middle), multiply(left, right))
                {
                    return false;
                }

                if multiply(add(left, middle), right)
                    != add(multiply(left, right), multiply(middle, right))
                {
                    return false;
                }
            }
        }
    }

    true
}

/// Returns whether `elements` with `add` and `multiply` form a ring.
#[must_use]
pub fn is_ring<T, Add, Mul>(elements: &[T], add: Add, multiply: Mul) -> bool
where
    T: Copy + PartialEq,
    Add: Fn(T, T) -> T + Copy,
    Mul: Fn(T, T) -> T + Copy,
{
    is_abelian_group(elements, add)
        && is_closed_under(elements, multiply)
        && is_associative(elements, multiply)
        && is_distributive_over(elements, multiply, add)
}

#[cfg(test)]
mod tests {
    use super::{
        has_inverses, identity_element, is_abelian_group, is_associative, is_closed_under,
        is_commutative, is_distributive_over, is_group, is_monoid, is_ring,
    };

    #[test]
    fn validates_modular_group_and_ring_examples() {
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
    fn rejects_operations_that_do_not_form_the_expected_structures() {
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
}
