use std::collections::HashMap;

use crate::error::GeodeError;

/// Finite hyper-Catalan type vectors of the form `[m2, m3, m4, ...]`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeVector {
    values: Vec<u64>,
}

impl TypeVector {
    /// Creates a validated finite type vector.
    ///
    /// The vector must contain at least one component, and the total face count
    /// must still fit in `u64` so `face_count()` remains exact.
    ///
    /// # Errors
    ///
    /// Returns [`GeodeError::EmptyTypeVector`] when `values` is empty.
    ///
    /// Returns [`GeodeError::InvalidInput`] when the total face count would no
    /// longer fit in `u64`.
    pub fn new(values: Vec<u64>) -> Result<Self, GeodeError> {
        if values.is_empty() {
            return Err(GeodeError::EmptyTypeVector);
        }

        values
            .iter()
            .try_fold(0_u64, |total, value| total.checked_add(*value))
            .ok_or(GeodeError::InvalidInput)?;

        Ok(Self { values })
    }

    /// Returns the underlying component slice.
    #[must_use]
    pub fn values(&self) -> &[u64] {
        &self.values
    }

    /// Returns the current dimension of the finite type vector.
    #[must_use]
    pub const fn dimension(&self) -> usize {
        self.values.len()
    }

    /// Returns the total face count.
    #[must_use]
    pub fn face_count(&self) -> u64 {
        face_count(self)
    }

    /// Returns whether all components are zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.values.iter().all(|value| *value == 0)
    }

    /// Returns a copy with one checked increment at `index`.
    ///
    /// # Errors
    ///
    /// Returns [`GeodeError::IndexOutOfBounds`] when `index` is not present.
    ///
    /// Returns [`GeodeError::ArithmeticOverflow`] when incrementing the chosen
    /// component would overflow `u64`.
    ///
    /// Returns [`GeodeError::InvalidInput`] when the resulting total face count
    /// no longer fits in `u64`.
    pub fn incremented(&self, index: usize) -> Result<Self, GeodeError> {
        let mut values = self.values.clone();
        let value = values.get_mut(index).ok_or(GeodeError::IndexOutOfBounds)?;
        *value = value.checked_add(1).ok_or(GeodeError::ArithmeticOverflow)?;
        Self::new(values)
    }

    /// Returns a copy with one decrement at `index`, or `None` when already zero.
    ///
    /// # Errors
    ///
    /// Returns [`GeodeError::IndexOutOfBounds`] when `index` is not present.
    ///
    /// Returns [`GeodeError::InvalidInput`] when the resulting total face count
    /// no longer fits in `u64`.
    pub fn decremented(&self, index: usize) -> Result<Option<Self>, GeodeError> {
        let mut values = self.values.clone();
        let value = values.get_mut(index).ok_or(GeodeError::IndexOutOfBounds)?;

        if *value == 0 {
            return Ok(None);
        }

        *value -= 1;
        Self::new(values).map(Some)
    }

    /// Removes trailing zeroes while keeping at least one component.
    #[must_use]
    pub fn trimmed(&self) -> Self {
        let mut values = self.values.clone();

        while values.len() > 1 && values.last() == Some(&0) {
            values.pop();
        }

        Self { values }
    }
}

/// Returns the total face count `F = m2 + m3 + m4 + ...`.
#[must_use]
pub fn face_count(m: &TypeVector) -> u64 {
    m.values.iter().copied().sum()
}

/// Returns `E = 1 + 2m2 + 3m3 + 4m4 + ...` using checked `u128` arithmetic.
///
/// # Errors
///
/// Returns [`GeodeError::ArithmeticOverflow`] when any intermediate weighted
/// sum exceeds `u128`.
pub fn polygon_edge_count(m: &TypeVector) -> Result<u128, GeodeError> {
    weighted_sum_with_offset(m, 1, 2)
}

/// Returns `V = 2 + m2 + 2m3 + 3m4 + ...` using checked `u128` arithmetic.
///
/// # Errors
///
/// Returns [`GeodeError::ArithmeticOverflow`] when any intermediate weighted
/// sum exceeds `u128`.
pub fn polygon_vertex_count(m: &TypeVector) -> Result<u128, GeodeError> {
    weighted_sum_with_offset(m, 2, 1)
}

/// Returns `n!` using checked `u128` arithmetic.
///
/// # Errors
///
/// Returns [`GeodeError::ArithmeticOverflow`] when the factorial no longer
/// fits in `u128`.
pub fn checked_factorial(n: u64) -> Result<u128, GeodeError> {
    let mut result = 1_u128;
    let mut factor = 2_u64;

    while factor <= n {
        result = result
            .checked_mul(u128::from(factor))
            .ok_or(GeodeError::ArithmeticOverflow)?;
        factor += 1;
    }

    Ok(result)
}

/// Returns the checked product of each factorial in `values`.
///
/// # Errors
///
/// Returns [`GeodeError::ArithmeticOverflow`] when any constituent factorial or
/// the final product no longer fits in `u128`.
pub fn checked_product_factorials(values: &[u64]) -> Result<u128, GeodeError> {
    let mut product = 1_u128;

    for value in values {
        product = product
            .checked_mul(checked_factorial(*value)?)
            .ok_or(GeodeError::ArithmeticOverflow)?;
    }

    Ok(product)
}

/// Returns an exact integer quotient.
///
/// # Errors
///
/// Returns [`GeodeError::DivisionNotExact`] when `denominator == 0` or the
/// division leaves a non-zero remainder.
pub const fn exact_divide(numerator: u128, denominator: u128) -> Result<u128, GeodeError> {
    if denominator == 0 || !numerator.is_multiple_of(denominator) {
        return Err(GeodeError::DivisionNotExact);
    }

    Ok(numerator / denominator)
}

/// Returns the finite-type hyper-Catalan coefficient for `m`.
///
/// # Errors
///
/// Returns [`GeodeError::ArithmeticOverflow`] when structural counts or
/// factorials exceed the exact integer range.
///
/// Returns [`GeodeError::DivisionNotExact`] when the closed form does not divide
/// exactly in `u128` arithmetic.
pub fn hyper_catalan(m: &TypeVector) -> Result<u128, GeodeError> {
    let edge_count = polygon_edge_count(m)?;
    let vertex_count = polygon_vertex_count(m)?;
    let numerator = checked_factorial(
        u64::try_from(edge_count - 1).map_err(|_| GeodeError::ArithmeticOverflow)?,
    )?;
    let vertex_factorial = checked_factorial(
        u64::try_from(vertex_count - 1).map_err(|_| GeodeError::ArithmeticOverflow)?,
    )?;
    let face_factorials = checked_product_factorials(m.values())?;
    let denominator = vertex_factorial
        .checked_mul(face_factorials)
        .ok_or(GeodeError::ArithmeticOverflow)?;

    exact_divide(numerator, denominator)
}

/// Returns the Geode coefficient `G[m]` for small exact inputs.
///
/// This direct version uses the defining recurrence and is intended only for
/// small vectors.
///
/// # Errors
///
/// Returns [`GeodeError::ArithmeticOverflow`] when the underlying hyper-Catalan
/// count overflows or the recurrence would subtract past zero.
///
/// Returns [`GeodeError::DivisionNotExact`] when the underlying hyper-Catalan
/// closed form does not divide exactly in `u128` arithmetic.
pub fn geode(m: &TypeVector) -> Result<u128, GeodeError> {
    geode_impl(m, None)
}

/// Returns the Geode coefficient `G[m]` using internal memoization.
///
/// # Errors
///
/// Returns the same errors as [`geode`].
pub fn geode_memoized(m: &TypeVector) -> Result<u128, GeodeError> {
    let mut memo = HashMap::new();
    geode_impl(m, Some(&mut memo))
}

/// Returns the one-dimensional Catalan value that matches `[n]`.
///
/// # Errors
///
/// Returns [`GeodeError::ArithmeticOverflow`] when the corresponding Catalan
/// number no longer fits in `u128`.
pub fn catalan_from_geode_dimension(n: u64) -> Result<u128, GeodeError> {
    use_catalan::catalan(n).map_err(|_| GeodeError::ArithmeticOverflow)
}

/// Returns the Geode coefficient along the first axis `[n]`.
///
/// # Errors
///
/// Returns any validation or arithmetic error produced while constructing the
/// axis vector or evaluating [`geode`].
pub fn geode_on_first_axis(n: u64) -> Result<u128, GeodeError> {
    geode(&TypeVector::new(vec![n])?)
}

/// Returns the diagonal Geode coefficient for `[n, n]`.
///
/// # Errors
///
/// Returns any validation or arithmetic error produced while constructing the
/// diagonal vector or evaluating [`geode`].
pub fn diagonal_geode_2d(n: u64) -> Result<u128, GeodeError> {
    geode(&TypeVector::new(vec![n, n])?)
}

/// Returns the diagonal Geode coefficient for `[n, n, n]`.
///
/// # Errors
///
/// Returns any validation or arithmetic error produced while constructing the
/// diagonal vector or evaluating [`geode`].
pub fn diagonal_geode_3d(n: u64) -> Result<u128, GeodeError> {
    geode(&TypeVector::new(vec![n, n, n])?)
}

/// Returns the diagonal Geode coefficient for `[n, n, n, n]`.
///
/// # Errors
///
/// Returns any validation or arithmetic error produced while constructing the
/// diagonal vector or evaluating [`geode`].
pub fn diagonal_geode_4d(n: u64) -> Result<u128, GeodeError> {
    geode(&TypeVector::new(vec![n, n, n, n])?)
}

fn weighted_sum_with_offset(
    m: &TypeVector,
    constant_term: u128,
    first_weight: u128,
) -> Result<u128, GeodeError> {
    let mut total = constant_term;

    for (index, value) in m.values.iter().enumerate() {
        let index_weight = u128::try_from(index).map_err(|_| GeodeError::ArithmeticOverflow)?;
        let weight = first_weight
            .checked_add(index_weight)
            .ok_or(GeodeError::ArithmeticOverflow)?;
        let contribution = weight
            .checked_mul(u128::from(*value))
            .ok_or(GeodeError::ArithmeticOverflow)?;
        total = total
            .checked_add(contribution)
            .ok_or(GeodeError::ArithmeticOverflow)?;
    }

    Ok(total)
}

fn geode_impl(
    m: &TypeVector,
    mut memo: Option<&mut HashMap<Vec<u64>, u128>>,
) -> Result<u128, GeodeError> {
    if let Some(cache) = &mut memo
        && let Some(value) = cache.get(m.values())
    {
        return Ok(*value);
    }

    let incremented = m.incremented(0)?;
    let mut result = hyper_catalan(&incremented)?;

    for index in 1..m.dimension() {
        if m.values()[index] == 0 {
            continue;
        }

        let shifted = incremented
            .decremented(index)?
            .ok_or(GeodeError::ArithmeticOverflow)?;
        let term = geode_impl(&shifted.trimmed(), memo.as_deref_mut())?;
        result = result
            .checked_sub(term)
            .ok_or(GeodeError::ArithmeticOverflow)?;
    }

    if let Some(cache) = memo {
        cache.insert(m.values.clone(), result);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use use_catalan::catalan;

    use super::{
        GeodeError, TypeVector, catalan_from_geode_dimension, checked_factorial,
        checked_product_factorials, diagonal_geode_2d, diagonal_geode_3d, diagonal_geode_4d,
        exact_divide, face_count, geode, geode_memoized, geode_on_first_axis, hyper_catalan,
        polygon_edge_count, polygon_vertex_count,
    };

    #[test]
    fn rejects_empty_type_vectors() {
        assert_eq!(TypeVector::new(vec![]), Err(GeodeError::EmptyTypeVector));
    }

    #[test]
    fn accepts_zero_type_vectors() -> Result<(), GeodeError> {
        let vector = TypeVector::new(vec![0])?;

        assert_eq!(vector.values(), &[0]);
        assert_eq!(vector.dimension(), 1);
        assert!(vector.is_zero());

        Ok(())
    }

    #[test]
    fn preserves_dimension_and_face_counts() -> Result<(), GeodeError> {
        let vector = TypeVector::new(vec![2, 1, 0])?;

        assert_eq!(vector.dimension(), 3);
        assert_eq!(vector.face_count(), 3);
        assert_eq!(face_count(&vector), 3);
        assert_eq!(vector.trimmed().dimension(), 2);

        Ok(())
    }

    #[test]
    fn computes_polygon_edge_counts() -> Result<(), GeodeError> {
        assert_eq!(polygon_edge_count(&TypeVector::new(vec![0])?)?, 1);
        assert_eq!(polygon_edge_count(&TypeVector::new(vec![2, 1])?)?, 8);

        Ok(())
    }

    #[test]
    fn computes_polygon_vertex_counts() -> Result<(), GeodeError> {
        assert_eq!(polygon_vertex_count(&TypeVector::new(vec![0])?)?, 2);
        assert_eq!(polygon_vertex_count(&TypeVector::new(vec![2, 1])?)?, 6);

        Ok(())
    }

    #[test]
    fn increments_and_decrements_components() -> Result<(), GeodeError> {
        let vector = TypeVector::new(vec![1, 2, 0])?;
        let decremented = vector.decremented(1)?;

        assert_eq!(vector.incremented(1)?.values(), &[1, 3, 0]);
        assert_eq!(decremented, Some(TypeVector::new(vec![1, 1, 0])?));

        Ok(())
    }

    #[test]
    fn decrementing_zero_component_returns_none() -> Result<(), GeodeError> {
        let vector = TypeVector::new(vec![1, 0])?;

        assert_eq!(vector.decremented(1)?, None);

        Ok(())
    }

    #[test]
    fn reports_invalid_indices() -> Result<(), GeodeError> {
        let vector = TypeVector::new(vec![1, 2])?;

        assert_eq!(vector.incremented(2), Err(GeodeError::IndexOutOfBounds));
        assert_eq!(vector.decremented(3), Err(GeodeError::IndexOutOfBounds));

        Ok(())
    }

    #[test]
    fn computes_checked_factorials() {
        assert_eq!(checked_factorial(0), Ok(1));
        assert_eq!(checked_factorial(1), Ok(1));
        assert_eq!(checked_factorial(5), Ok(120));
    }

    #[test]
    fn computes_factorial_products() {
        assert_eq!(checked_product_factorials(&[0, 1, 3]), Ok(6));
        assert_eq!(checked_product_factorials(&[2, 2]), Ok(4));
    }

    #[test]
    fn performs_exact_division() {
        assert_eq!(exact_divide(12, 3), Ok(4));
        assert_eq!(exact_divide(120, 10), Ok(12));
    }

    #[test]
    fn rejects_non_exact_division() {
        assert_eq!(exact_divide(10, 3), Err(GeodeError::DivisionNotExact));
        assert_eq!(exact_divide(10, 0), Err(GeodeError::DivisionNotExact));
    }

    #[test]
    fn one_dimensional_hyper_catalan_matches_catalan() -> Result<(), GeodeError> {
        for n in 0_u64..=5 {
            let vector = TypeVector::new(vec![n])?;
            let catalan_value = catalan(n).map_err(|_| GeodeError::ArithmeticOverflow)?;

            assert_eq!(hyper_catalan(&vector)?, catalan_value);
            assert_eq!(catalan_from_geode_dimension(n)?, catalan_value);
        }

        Ok(())
    }

    #[test]
    fn computes_small_multidimensional_hyper_catalan_values() -> Result<(), GeodeError> {
        assert_eq!(hyper_catalan(&TypeVector::new(vec![1, 1])?)?, 5);
        assert_eq!(hyper_catalan(&TypeVector::new(vec![2, 1])?)?, 21);
        assert_eq!(hyper_catalan(&TypeVector::new(vec![1, 0, 1])?)?, 6);

        Ok(())
    }

    #[test]
    fn computes_small_geode_values() -> Result<(), GeodeError> {
        assert_eq!(geode(&TypeVector::new(vec![0])?)?, 1);
        assert_eq!(geode_on_first_axis(1)?, 2);
        assert_eq!(geode(&TypeVector::new(vec![1, 0])?)?, 2);
        assert_eq!(geode(&TypeVector::new(vec![0, 1])?)?, 3);
        assert_eq!(geode(&TypeVector::new(vec![1, 1])?)?, 16);

        Ok(())
    }

    #[test]
    fn memoized_geode_matches_direct_computation() -> Result<(), GeodeError> {
        let vectors = [
            TypeVector::new(vec![0])?,
            TypeVector::new(vec![1])?,
            TypeVector::new(vec![2])?,
            TypeVector::new(vec![0, 1])?,
            TypeVector::new(vec![1, 1])?,
            TypeVector::new(vec![2, 1])?,
            TypeVector::new(vec![1, 0, 1])?,
        ];

        for vector in vectors {
            assert_eq!(geode_memoized(&vector)?, geode(&vector)?);
        }

        Ok(())
    }

    #[test]
    fn diagonal_helpers_are_deterministic() -> Result<(), GeodeError> {
        assert_eq!(diagonal_geode_2d(0)?, geode(&TypeVector::new(vec![0, 0])?)?);
        assert_eq!(diagonal_geode_2d(1)?, geode(&TypeVector::new(vec![1, 1])?)?);
        assert_eq!(
            diagonal_geode_3d(1)?,
            geode(&TypeVector::new(vec![1, 1, 1])?)?
        );
        assert_eq!(
            diagonal_geode_4d(1)?,
            geode(&TypeVector::new(vec![1, 1, 1, 1])?)?
        );

        Ok(())
    }

    #[test]
    fn reports_overflow_where_practical() {
        assert_eq!(checked_factorial(35), Err(GeodeError::ArithmeticOverflow));
        assert_eq!(
            TypeVector::new(vec![u64::MAX, 1]),
            Err(GeodeError::InvalidInput)
        );
    }
}
