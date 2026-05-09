/// Removes trailing zero coefficients from the given vector.
///
/// Both `0.0` and `-0.0` are treated as zero and stripped. The zero series is
/// represented as an empty vector.
pub(crate) fn normalize(mut coefficients: Vec<f64>) -> Vec<f64> {
    while coefficients.last().is_some_and(|c| *c == 0.0) {
        coefficients.pop();
    }
    coefficients
}

/// A finite or truncated power series stored as `f64` coefficients in ascending degree order.
///
/// # Coefficient layout
///
/// Index `i` holds the coefficient of `x^i`. For example, `[1.0, 2.0, 3.0]` represents
/// `1 + 2x + 3x²`.
///
/// # Zero representation
///
/// The zero series is stored as an empty coefficient vector. Any series whose coefficients
/// normalize to all zeros becomes the zero series. [`Series::is_zero`] and [`Series::is_empty`]
/// both return `true` for the zero series, and [`Series::order`] returns `None`.
///
/// # Normalization
///
/// All constructors strip trailing zero coefficients. This ensures that `len() - 1` always
/// equals the true degree for non-zero series.
///
/// # Examples
///
/// ```rust
/// use use_series::Series;
///
/// let s = Series::new(vec![1.0, 2.0, 3.0]); // 1 + 2x + 3x²
/// assert_eq!(s.evaluate(2.0), 17.0);          // 1 + 4 + 12 = 17
/// assert_eq!(s.order(), Some(2));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Series {
    coefficients: Vec<f64>,
}

impl Series {
    /// Creates a new series from the given coefficient vector.
    ///
    /// Coefficients are given in ascending degree order: index `i` holds the coefficient of
    /// `x^i`. Trailing zeros are removed during construction.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// let s = Series::new(vec![1.0, 2.0, 3.0]); // 1 + 2x + 3x²
    /// assert_eq!(s.coefficients(), &[1.0, 2.0, 3.0]);
    /// ```
    #[must_use]
    pub fn new(coefficients: Vec<f64>) -> Self {
        Self {
            coefficients: normalize(coefficients),
        }
    }

    /// Creates the zero series (empty coefficient vector).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// let z = Series::zero();
    /// assert!(z.is_zero());
    /// assert_eq!(z.order(), None);
    /// ```
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            coefficients: Vec::new(),
        }
    }

    /// Creates a constant series with the given value.
    ///
    /// If `value` is `0.0`, returns the zero series.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// let c = Series::constant(5.0);
    /// assert_eq!(c.coefficients(), &[5.0]);
    /// assert_eq!(c.order(), Some(0));
    /// ```
    #[must_use]
    pub fn constant(value: f64) -> Self {
        Self::new(vec![value])
    }

    /// Creates a series from the given coefficient vector.
    ///
    /// This is an alias for [`Series::new`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// let s = Series::from_coefficients(vec![1.0, 0.0, 2.0]); // 1 + 2x²
    /// assert_eq!(s.coefficients(), &[1.0, 0.0, 2.0]);
    /// ```
    #[must_use]
    pub fn from_coefficients(coefficients: Vec<f64>) -> Self {
        Self::new(coefficients)
    }

    /// Returns the stored coefficients in ascending degree order.
    ///
    /// The zero series returns an empty slice. For non-zero series the last element is
    /// always non-zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// let s = Series::new(vec![1.0, 2.0, 3.0]);
    /// assert_eq!(s.coefficients(), &[1.0, 2.0, 3.0]);
    /// ```
    #[must_use]
    pub fn coefficients(&self) -> &[f64] {
        &self.coefficients
    }

    /// Returns the coefficient at the given order, or `0.0` if the order is out of range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// let s = Series::new(vec![1.0, 2.0, 3.0]);
    /// assert_eq!(s.coefficient(0), 1.0);
    /// assert_eq!(s.coefficient(2), 3.0);
    /// assert_eq!(s.coefficient(5), 0.0); // out of range
    /// ```
    #[must_use]
    pub fn coefficient(&self, order: usize) -> f64 {
        self.coefficients.get(order).copied().unwrap_or(0.0)
    }

    /// Returns the order (highest non-zero degree) of the series, or `None` for the zero series.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// assert_eq!(Series::zero().order(), None);
    /// assert_eq!(Series::constant(1.0).order(), Some(0));
    /// assert_eq!(Series::new(vec![1.0, 2.0, 3.0]).order(), Some(2));
    /// ```
    #[must_use]
    pub const fn order(&self) -> Option<usize> {
        if self.is_zero() {
            None
        } else {
            Some(self.coefficients.len() - 1)
        }
    }

    /// Returns the number of stored coefficients.
    ///
    /// For non-zero series this is `order() + 1`. For the zero series this is `0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// assert_eq!(Series::zero().len(), 0);
    /// assert_eq!(Series::new(vec![1.0, 2.0, 3.0]).len(), 3);
    /// ```
    #[must_use]
    pub const fn len(&self) -> usize {
        self.coefficients.len()
    }

    /// Returns `true` if the series has no stored coefficients.
    ///
    /// Equivalent to [`Series::is_zero`] since normalization removes all trailing zeros.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// assert!(Series::zero().is_empty());
    /// assert!(!Series::constant(1.0).is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.coefficients.is_empty()
    }

    /// Returns `true` if the series is the zero series.
    ///
    /// The zero series has an empty coefficient vector after normalization.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// assert!(Series::zero().is_zero());
    /// assert!(Series::new(vec![0.0, 0.0]).is_zero()); // normalized to zero
    /// assert!(!Series::constant(1.0).is_zero());
    /// ```
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.coefficients.is_empty()
    }

    /// Evaluates the series at `x` using Horner's method.
    ///
    /// Horner's method computes `(...((aₙ · x + aₙ₋₁) · x + aₙ₋₂) · ... + a₀)` for numerical
    /// stability and efficiency. The zero series evaluates to `0.0` for any `x`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// let s = Series::new(vec![1.0, 2.0, 3.0]); // 1 + 2x + 3x²
    /// assert_eq!(s.evaluate(2.0), 17.0);          // 1 + 4 + 12 = 17
    /// assert_eq!(s.evaluate(0.0), 1.0);
    /// assert_eq!(Series::zero().evaluate(99.0), 0.0);
    /// ```
    #[must_use]
    pub fn evaluate(&self, x: f64) -> f64 {
        self.coefficients
            .iter()
            .rev()
            .fold(0.0, |acc, &c| acc.mul_add(x, c))
    }

    /// Returns the series truncated to at most `max_order` degree.
    ///
    /// Keeps all terms from degree 0 through `max_order` inclusive. If the series has fewer
    /// terms than `max_order + 1`, the full series is returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// let s = Series::new(vec![1.0, 2.0, 3.0, 4.0]); // 1 + 2x + 3x² + 4x³
    /// let t = s.truncate(2);                           // 1 + 2x + 3x²
    /// assert_eq!(t.coefficients(), &[1.0, 2.0, 3.0]);
    /// ```
    #[must_use]
    pub fn truncate(&self, max_order: usize) -> Self {
        let end = (max_order + 1).min(self.coefficients.len());
        Self::new(self.coefficients[..end].to_vec())
    }

    /// Returns the elementwise sum of this series and `other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// let a = Series::new(vec![1.0, 2.0]);
    /// let b = Series::new(vec![3.0, 0.0, 4.0]);
    /// let sum = a.add(&b);
    /// assert_eq!(sum.coefficients(), &[4.0, 2.0, 4.0]);
    /// ```
    #[must_use]
    pub fn add(&self, other: &Self) -> Self {
        let len = self.coefficients.len().max(other.coefficients.len());
        let coeffs: Vec<f64> = (0..len)
            .map(|i| self.coefficient(i) + other.coefficient(i))
            .collect();
        Self::new(coeffs)
    }

    /// Returns the elementwise difference of this series and `other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// let a = Series::new(vec![4.0, 6.0]);
    /// let b = Series::new(vec![1.0, 2.0]);
    /// let diff = a.sub(&b);
    /// assert_eq!(diff.coefficients(), &[3.0, 4.0]);
    /// ```
    #[must_use]
    pub fn sub(&self, other: &Self) -> Self {
        let len = self.coefficients.len().max(other.coefficients.len());
        let coeffs: Vec<f64> = (0..len)
            .map(|i| self.coefficient(i) - other.coefficient(i))
            .collect();
        Self::new(coeffs)
    }

    /// Returns the Cauchy product (polynomial multiplication) of this series and `other`.
    ///
    /// The coefficient of `x^k` in the result is `Σ aᵢ · bⱼ` for all `i + j = k`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// let a = Series::new(vec![1.0, 2.0]);   // 1 + 2x
    /// let b = Series::new(vec![3.0, 4.0]);   // 3 + 4x
    /// let p = a.mul(&b);                      // (1 + 2x)(3 + 4x) = 3 + 10x + 8x²
    /// assert_eq!(p.coefficients(), &[3.0, 10.0, 8.0]);
    /// ```
    #[must_use]
    pub fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        let result_len = self.coefficients.len() + other.coefficients.len() - 1;
        let mut coeffs = vec![0.0_f64; result_len];
        for (i, &a) in self.coefficients.iter().enumerate() {
            for (j, &b) in other.coefficients.iter().enumerate() {
                coeffs[i + j] += a * b;
            }
        }
        Self::new(coeffs)
    }

    /// Returns the series with every coefficient multiplied by `scalar`.
    ///
    /// Scaling by `0.0` produces the zero series.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// let s = Series::new(vec![1.0, 2.0, 3.0]);
    /// let scaled = s.scale(2.0);
    /// assert_eq!(scaled.coefficients(), &[2.0, 4.0, 6.0]);
    /// ```
    #[must_use]
    pub fn scale(&self, scalar: f64) -> Self {
        let coeffs: Vec<f64> = self.coefficients.iter().map(|&c| c * scalar).collect();
        Self::new(coeffs)
    }

    /// Returns the series shifted by `amount` degrees (multiplied by `x^amount`).
    ///
    /// This prepends `amount` zero coefficients. Shifting the zero series returns the zero series.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// let s = Series::new(vec![1.0, 2.0]); // 1 + 2x
    /// let shifted = s.shift(2);             // x² + 2x³
    /// assert_eq!(shifted.coefficients(), &[0.0, 0.0, 1.0, 2.0]);
    /// ```
    #[must_use]
    pub fn shift(&self, amount: usize) -> Self {
        if self.is_zero() {
            return Self::zero();
        }
        let mut coeffs = vec![0.0_f64; amount];
        coeffs.extend_from_slice(&self.coefficients);
        Self::new(coeffs)
    }

    /// Returns the formal derivative of the series.
    ///
    /// `d/dx Σ aᵢ xⁱ = Σ i · aᵢ xⁱ⁻¹`. The derivative of the zero series or a constant series
    /// is the zero series.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// let s = Series::new(vec![1.0, 2.0, 3.0]); // 1 + 2x + 3x²
    /// let d = s.derivative();                     // 2 + 6x
    /// assert_eq!(d.coefficients(), &[2.0, 6.0]);
    /// ```
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn derivative(&self) -> Self {
        if self.coefficients.len() <= 1 {
            return Self::zero();
        }
        let coeffs: Vec<f64> = self.coefficients[1..]
            .iter()
            .enumerate()
            .map(|(i, &c)| c * (i + 1) as f64)
            .collect();
        Self::new(coeffs)
    }

    /// Returns the formal integral of the series with `constant` as the constant term.
    ///
    /// `∫ Σ aᵢ xⁱ dx = constant + Σ (aᵢ / (i + 1)) xⁱ⁺¹`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_series::Series;
    ///
    /// let s = Series::new(vec![1.0, 2.0, 3.0]); // 1 + 2x + 3x²
    /// let i = s.integral(0.0);                   // x + x² + x³
    /// assert_eq!(i.coefficients(), &[0.0, 1.0, 1.0, 1.0]);
    /// ```
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn integral(&self, constant: f64) -> Self {
        let mut coeffs = Vec::with_capacity(self.coefficients.len() + 1);
        coeffs.push(constant);
        for (i, &c) in self.coefficients.iter().enumerate() {
            coeffs.push(c / (i + 1) as f64);
        }
        Self::new(coeffs)
    }
}

#[cfg(test)]
mod tests {
    use super::{Series, normalize};

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1.0e-12
    }

    // ── normalization helpers ──────────────────────────────────────────────

    #[test]
    fn normalize_strips_trailing_zeros() {
        assert_eq!(normalize(vec![1.0, 2.0, 0.0, 0.0]), vec![1.0, 2.0]);
    }

    #[test]
    fn normalize_all_zeros_produces_empty() {
        assert!(normalize(vec![0.0, 0.0, 0.0]).is_empty());
    }

    #[test]
    fn normalize_empty_is_empty() {
        assert!(normalize(vec![]).is_empty());
    }

    #[test]
    fn normalize_preserves_internal_zeros() {
        assert_eq!(normalize(vec![1.0, 0.0, 3.0]), vec![1.0, 0.0, 3.0]);
    }

    // ── constructors ──────────────────────────────────────────────────────

    #[test]
    fn new_normalizes_trailing_zeros() {
        let s = Series::new(vec![1.0, 2.0, 0.0]);
        assert_eq!(s.coefficients(), &[1.0, 2.0]);
    }

    #[test]
    fn zero_is_empty() {
        let z = Series::zero();
        assert!(z.is_empty());
        assert!(z.is_zero());
        assert_eq!(z.len(), 0);
        assert_eq!(z.order(), None);
    }

    #[test]
    fn constant_stores_single_coefficient() {
        let c = Series::constant(5.0);
        assert_eq!(c.coefficients(), &[5.0]);
        assert_eq!(c.order(), Some(0));
    }

    #[test]
    fn constant_zero_produces_zero_series() {
        let c = Series::constant(0.0);
        assert!(c.is_zero());
    }

    #[test]
    fn from_coefficients_aliases_new() {
        let a = Series::new(vec![1.0, 2.0, 3.0]);
        let b = Series::from_coefficients(vec![1.0, 2.0, 3.0]);
        assert_eq!(a, b);
    }

    // ── accessors ─────────────────────────────────────────────────────────

    #[test]
    fn coefficient_returns_stored_value() {
        let s = Series::new(vec![1.0, 2.0, 3.0]);
        assert!(approx_eq(s.coefficient(0), 1.0));
        assert!(approx_eq(s.coefficient(1), 2.0));
        assert!(approx_eq(s.coefficient(2), 3.0));
    }

    #[test]
    fn coefficient_returns_zero_for_out_of_range() {
        let s = Series::new(vec![1.0, 2.0]);
        assert!(approx_eq(s.coefficient(5), 0.0));
    }

    #[test]
    fn coefficient_zero_series_always_returns_zero() {
        let z = Series::zero();
        assert!(approx_eq(z.coefficient(0), 0.0));
        assert!(approx_eq(z.coefficient(100), 0.0));
    }

    #[test]
    fn order_none_for_zero_series() {
        assert_eq!(Series::zero().order(), None);
    }

    #[test]
    fn order_zero_for_constant() {
        assert_eq!(Series::constant(1.0).order(), Some(0));
    }

    #[test]
    fn order_matches_highest_degree() {
        let s = Series::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(s.order(), Some(2));
    }

    #[test]
    fn len_matches_coefficient_count() {
        assert_eq!(Series::zero().len(), 0);
        assert_eq!(Series::new(vec![1.0, 2.0, 3.0]).len(), 3);
    }

    #[test]
    fn is_empty_true_for_zero_series() {
        assert!(Series::zero().is_empty());
    }

    #[test]
    fn is_empty_false_for_nonzero() {
        assert!(!Series::constant(1.0).is_empty());
    }

    // ── evaluate ──────────────────────────────────────────────────────────

    #[test]
    fn evaluate_zero_series_is_zero() {
        assert!(approx_eq(Series::zero().evaluate(42.0), 0.0));
    }

    #[test]
    fn evaluate_constant() {
        assert!(approx_eq(Series::constant(7.0).evaluate(99.0), 7.0));
    }

    #[test]
    fn evaluate_linear() {
        let s = Series::new(vec![1.0, 2.0]); // 1 + 2x
        assert!(approx_eq(s.evaluate(3.0), 7.0)); // 1 + 6 = 7
    }

    #[test]
    fn evaluate_quadratic() {
        let s = Series::new(vec![1.0, 2.0, 3.0]); // 1 + 2x + 3x²
        assert!(approx_eq(s.evaluate(2.0), 17.0)); // 1 + 4 + 12 = 17
    }

    #[test]
    fn evaluate_at_zero() {
        let s = Series::new(vec![5.0, 99.0, 99.0]);
        assert!(approx_eq(s.evaluate(0.0), 5.0));
    }

    // ── truncate ──────────────────────────────────────────────────────────

    #[test]
    fn truncate_keeps_terms_through_max_order() {
        let s = Series::new(vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(s.truncate(2).coefficients(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn truncate_to_full_length_returns_same() {
        let s = Series::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(s.truncate(10).coefficients(), s.coefficients());
    }

    #[test]
    fn truncate_to_zero_keeps_constant() {
        let s = Series::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(s.truncate(0).coefficients(), &[1.0]);
    }

    #[test]
    fn truncate_zero_series_is_zero() {
        assert!(Series::zero().truncate(5).is_zero());
    }

    // ── add ───────────────────────────────────────────────────────────────

    #[test]
    fn add_same_length() {
        let a = Series::new(vec![1.0, 2.0]);
        let b = Series::new(vec![3.0, 4.0]);
        assert_eq!(a.add(&b).coefficients(), &[4.0, 6.0]);
    }

    #[test]
    fn add_different_lengths() {
        let a = Series::new(vec![1.0, 2.0]);
        let b = Series::new(vec![3.0, 0.0, 5.0]);
        assert_eq!(a.add(&b).coefficients(), &[4.0, 2.0, 5.0]);
    }

    #[test]
    fn add_with_zero_series() {
        let a = Series::new(vec![1.0, 2.0]);
        assert_eq!(a.add(&Series::zero()), a);
    }

    #[test]
    fn add_canceling_coefficients_normalizes() {
        let a = Series::new(vec![1.0, 2.0]);
        let b = Series::new(vec![0.0, -2.0]);
        assert_eq!(a.add(&b).coefficients(), &[1.0]);
    }

    // ── sub ───────────────────────────────────────────────────────────────

    #[test]
    fn sub_same_length() {
        let a = Series::new(vec![4.0, 6.0]);
        let b = Series::new(vec![1.0, 2.0]);
        assert_eq!(a.sub(&b).coefficients(), &[3.0, 4.0]);
    }

    #[test]
    fn sub_self_is_zero() {
        let a = Series::new(vec![1.0, 2.0, 3.0]);
        assert!(a.sub(&a).is_zero());
    }

    #[test]
    fn sub_zero_series() {
        let a = Series::new(vec![1.0, 2.0]);
        assert_eq!(a.sub(&Series::zero()), a);
    }

    // ── mul ───────────────────────────────────────────────────────────────

    #[test]
    fn mul_two_linears() {
        let a = Series::new(vec![1.0, 2.0]); // 1 + 2x
        let b = Series::new(vec![3.0, 4.0]); // 3 + 4x
        // (1 + 2x)(3 + 4x) = 3 + 10x + 8x²
        assert_eq!(a.mul(&b).coefficients(), &[3.0, 10.0, 8.0]);
    }

    #[test]
    fn mul_by_zero_series_is_zero() {
        let a = Series::new(vec![1.0, 2.0, 3.0]);
        assert!(a.mul(&Series::zero()).is_zero());
        assert!(Series::zero().mul(&a).is_zero());
    }

    #[test]
    fn mul_by_constant_scales() {
        let a = Series::new(vec![1.0, 2.0, 3.0]);
        let two = Series::constant(2.0);
        assert_eq!(a.mul(&two).coefficients(), &[2.0, 4.0, 6.0]);
    }

    #[test]
    fn mul_result_degree_is_sum_of_degrees() {
        let a = Series::new(vec![1.0, 1.0]); // degree 1
        let b = Series::new(vec![1.0, 1.0]); // degree 1
        assert_eq!(a.mul(&b).order(), Some(2)); // degree 2
    }

    // ── scale ─────────────────────────────────────────────────────────────

    #[test]
    fn scale_multiplies_all_coefficients() {
        let s = Series::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(s.scale(2.0).coefficients(), &[2.0, 4.0, 6.0]);
    }

    #[test]
    fn scale_by_zero_produces_zero_series() {
        let s = Series::new(vec![1.0, 2.0, 3.0]);
        assert!(s.scale(0.0).is_zero());
    }

    #[test]
    fn scale_zero_series_is_zero() {
        assert!(Series::zero().scale(5.0).is_zero());
    }

    // ── shift ─────────────────────────────────────────────────────────────

    #[test]
    fn shift_prepends_zeros() {
        let s = Series::new(vec![1.0, 2.0]); // 1 + 2x
        let shifted = s.shift(2); // x² + 2x³
        assert_eq!(shifted.coefficients(), &[0.0, 0.0, 1.0, 2.0]);
    }

    #[test]
    fn shift_by_zero_is_identity() {
        let s = Series::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(s.shift(0), s);
    }

    #[test]
    fn shift_zero_series_is_zero() {
        assert!(Series::zero().shift(5).is_zero());
    }

    // ── derivative ────────────────────────────────────────────────────────

    #[test]
    fn derivative_of_constant_is_zero() {
        assert!(Series::constant(5.0).derivative().is_zero());
    }

    #[test]
    fn derivative_of_zero_is_zero() {
        assert!(Series::zero().derivative().is_zero());
    }

    #[test]
    fn derivative_of_linear() {
        let s = Series::new(vec![3.0, 4.0]); // 3 + 4x
        assert_eq!(s.derivative().coefficients(), &[4.0]); // 4
    }

    #[test]
    fn derivative_of_quadratic() {
        let s = Series::new(vec![1.0, 2.0, 3.0]); // 1 + 2x + 3x²
        assert_eq!(s.derivative().coefficients(), &[2.0, 6.0]); // 2 + 6x
    }

    #[test]
    fn derivative_lowers_degree_by_one() {
        let s = Series::new(vec![1.0, 1.0, 1.0, 1.0]); // degree 3
        assert_eq!(s.derivative().order(), Some(2)); // degree 2
    }

    // ── integral ──────────────────────────────────────────────────────────

    #[test]
    fn integral_of_zero_series_is_constant() {
        let i = Series::zero().integral(7.0);
        assert_eq!(i.coefficients(), &[7.0]);
    }

    #[test]
    fn integral_of_constant() {
        let c = Series::constant(1.0); // 1
        let i = c.integral(0.0); // x
        assert_eq!(i.coefficients(), &[0.0, 1.0]);
    }

    #[test]
    fn integral_of_linear() {
        let s = Series::new(vec![1.0, 2.0]); // 1 + 2x
        let i = s.integral(0.0); // x + x²
        assert_eq!(i.coefficients(), &[0.0, 1.0, 1.0]);
    }

    #[test]
    fn integral_of_quadratic() {
        let s = Series::new(vec![1.0, 2.0, 3.0]); // 1 + 2x + 3x²
        let i = s.integral(0.0); // x + x² + x³
        assert_eq!(i.coefficients(), &[0.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn integral_with_nonzero_constant() {
        let s = Series::new(vec![2.0]); // 2
        let i = s.integral(5.0); // 5 + 2x
        assert_eq!(i.coefficients(), &[5.0, 2.0]);
    }

    #[test]
    fn derivative_of_integral_is_identity() {
        let s = Series::new(vec![1.0, 2.0, 3.0]);
        let i = s.integral(0.0);
        let d = i.derivative();
        // floating-point comparison with tolerance
        for (a, b) in s.coefficients().iter().zip(d.coefficients()) {
            assert!((a - b).abs() < 1.0e-12, "expected {a}, got {b}");
        }
        assert_eq!(d.len(), s.len());
    }
}
