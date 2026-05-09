/// A polynomial with `f64` coefficients stored in ascending degree order.
///
/// Coefficients are stored so that `coefficients[i]` is the coefficient of
/// `x^i`. For example, `[1.0, 2.0, 3.0]` represents `1 + 2x + 3x²`.
///
/// Trailing zero coefficients are always stripped on construction so that
/// internal representation is canonical. The zero polynomial is stored as
/// an empty slice (`coefficients.is_empty()`) and `degree()` returns `None`
/// for it.
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    /// Coefficients in ascending degree order, trailing zeros stripped.
    coefficients: Vec<f64>,
}

impl Polynomial {
    /// Creates a polynomial from a slice of coefficients in ascending degree
    /// order.
    ///
    /// `coefficients[i]` is the coefficient of `x^i`. Trailing zero
    /// coefficients are stripped automatically.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_polynomial::Polynomial;
    ///
    /// let p = Polynomial::new(vec![1.0, 2.0, 3.0]);
    /// assert_eq!(p.coefficients(), &[1.0, 2.0, 3.0]);
    ///
    /// // Trailing zeros are removed.
    /// let q = Polynomial::new(vec![1.0, 2.0, 0.0, 0.0]);
    /// assert_eq!(q.coefficients(), &[1.0, 2.0]);
    /// ```
    #[must_use]
    pub fn new(coefficients: Vec<f64>) -> Self {
        Self {
            coefficients: Self::normalize(coefficients),
        }
    }

    /// Returns the zero polynomial.
    ///
    /// The zero polynomial has no degree and evaluates to `0.0` everywhere.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_polynomial::Polynomial;
    ///
    /// let p = Polynomial::zero();
    /// assert!(p.is_zero());
    /// assert_eq!(p.degree(), None);
    /// ```
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            coefficients: Vec::new(),
        }
    }

    /// Creates a constant polynomial `c`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_polynomial::Polynomial;
    ///
    /// let p = Polynomial::constant(5.0);
    /// assert_eq!(p.evaluate(0.0), 5.0);
    /// assert_eq!(p.evaluate(99.0), 5.0);
    /// assert_eq!(p.degree(), Some(0));
    /// ```
    #[must_use]
    pub fn constant(value: f64) -> Self {
        if value == 0.0 {
            Self::zero()
        } else {
            Self::new(vec![value])
        }
    }

    /// Creates the linear polynomial `a + bx`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_polynomial::Polynomial;
    ///
    /// let p = Polynomial::linear(3.0, 2.0);
    /// assert_eq!(p.evaluate(1.0), 5.0);
    /// assert_eq!(p.degree(), Some(1));
    /// ```
    #[must_use]
    pub fn linear(a: f64, b: f64) -> Self {
        Self::new(vec![a, b])
    }

    /// Creates the quadratic polynomial `a + bx + cx²`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_polynomial::Polynomial;
    ///
    /// let p = Polynomial::quadratic(1.0, -3.0, 2.0);
    /// assert_eq!(p.evaluate(0.0), 1.0);
    /// assert_eq!(p.evaluate(1.0), 0.0);
    /// assert_eq!(p.evaluate(2.0), 3.0);
    /// assert_eq!(p.degree(), Some(2));
    /// ```
    #[must_use]
    pub fn quadratic(a: f64, b: f64, c: f64) -> Self {
        Self::new(vec![a, b, c])
    }

    /// Returns the degree of the polynomial, or `None` for the zero
    /// polynomial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_polynomial::Polynomial;
    ///
    /// assert_eq!(Polynomial::zero().degree(), None);
    /// assert_eq!(Polynomial::constant(3.0).degree(), Some(0));
    /// assert_eq!(Polynomial::linear(1.0, 2.0).degree(), Some(1));
    /// assert_eq!(Polynomial::quadratic(1.0, 0.0, 1.0).degree(), Some(2));
    /// ```
    #[must_use]
    pub const fn degree(&self) -> Option<usize> {
        if self.coefficients.is_empty() {
            None
        } else {
            Some(self.coefficients.len() - 1)
        }
    }

    /// Returns all coefficients in ascending degree order.
    ///
    /// Returns an empty slice for the zero polynomial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_polynomial::Polynomial;
    ///
    /// let p = Polynomial::new(vec![1.0, 2.0, 3.0]);
    /// assert_eq!(p.coefficients(), &[1.0, 2.0, 3.0]);
    ///
    /// assert_eq!(Polynomial::zero().coefficients(), &[] as &[f64]);
    /// ```
    #[must_use]
    pub fn coefficients(&self) -> &[f64] {
        &self.coefficients
    }

    /// Returns the coefficient of `x^degree`, or `0.0` when the degree is
    /// beyond the polynomial's degree.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_polynomial::Polynomial;
    ///
    /// let p = Polynomial::new(vec![1.0, 2.0, 3.0]);
    /// assert_eq!(p.coefficient(0), 1.0);
    /// assert_eq!(p.coefficient(1), 2.0);
    /// assert_eq!(p.coefficient(2), 3.0);
    /// assert_eq!(p.coefficient(9), 0.0);
    /// ```
    #[must_use]
    pub fn coefficient(&self, degree: usize) -> f64 {
        self.coefficients.get(degree).copied().unwrap_or(0.0)
    }

    /// Evaluates the polynomial at `x` using Horner's method.
    ///
    /// Returns `0.0` for the zero polynomial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_polynomial::Polynomial;
    ///
    /// let p = Polynomial::quadratic(1.0, -3.0, 2.0);
    /// assert_eq!(p.evaluate(5.0), 36.0);  // 1 - 15 + 50
    /// assert_eq!(Polynomial::zero().evaluate(99.0), 0.0);
    /// ```
    #[must_use]
    pub fn evaluate(&self, x: f64) -> f64 {
        // Horner's method: accumulate from the highest-degree coefficient down.
        self.coefficients
            .iter()
            .rev()
            .fold(0.0, |acc, &coeff| acc.mul_add(x, coeff))
    }

    /// Returns the formal derivative of the polynomial.
    ///
    /// The derivative of the zero polynomial is the zero polynomial.
    /// The derivative of a constant polynomial is the zero polynomial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_polynomial::Polynomial;
    ///
    /// // d/dx (1 - 3x + 2x²) = -3 + 4x
    /// let p = Polynomial::quadratic(1.0, -3.0, 2.0);
    /// let d = p.derivative();
    /// assert_eq!(d.coefficients(), &[-3.0, 4.0]);
    ///
    /// assert!(Polynomial::zero().derivative().is_zero());
    /// assert!(Polynomial::constant(5.0).derivative().is_zero());
    /// ```
    #[must_use]
    pub fn derivative(&self) -> Self {
        if self.coefficients.len() <= 1 {
            return Self::zero();
        }

        // Multiply each coefficient c_i by its degree i (i >= 1).
        // Use a running f64 counter to avoid a usize-to-f64 cast.
        let mut degree = 1.0_f64;
        let derived = self.coefficients[1..]
            .iter()
            .map(|&c| {
                let coeff = c * degree;
                degree += 1.0;
                coeff
            })
            .collect();

        Self::new(derived)
    }

    /// Returns the sum of this polynomial and `other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_polynomial::Polynomial;
    ///
    /// let p = Polynomial::linear(1.0, 2.0);
    /// let q = Polynomial::linear(3.0, 4.0);
    /// assert_eq!(p.add(&q).coefficients(), &[4.0, 6.0]);
    /// ```
    #[must_use]
    pub fn add(&self, other: &Self) -> Self {
        let len = self.coefficients.len().max(other.coefficients.len());
        let summed = (0..len)
            .map(|i| self.coefficient(i) + other.coefficient(i))
            .collect();
        Self::new(summed)
    }

    /// Returns the difference of this polynomial minus `other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_polynomial::Polynomial;
    ///
    /// let p = Polynomial::linear(5.0, 6.0);
    /// let q = Polynomial::linear(1.0, 2.0);
    /// assert_eq!(p.sub(&q).coefficients(), &[4.0, 4.0]);
    /// ```
    #[must_use]
    pub fn sub(&self, other: &Self) -> Self {
        let len = self.coefficients.len().max(other.coefficients.len());
        let diff = (0..len)
            .map(|i| self.coefficient(i) - other.coefficient(i))
            .collect();
        Self::new(diff)
    }

    /// Returns the product of this polynomial and `other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_polynomial::Polynomial;
    ///
    /// // (1 + x) * (1 - x) = 1 - x²
    /// let p = Polynomial::linear(1.0, 1.0);
    /// let q = Polynomial::linear(1.0, -1.0);
    /// assert_eq!(p.mul(&q).coefficients(), &[1.0, 0.0, -1.0]);
    /// ```
    #[must_use]
    pub fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }

        let result_len = self.coefficients.len() + other.coefficients.len() - 1;
        let mut product = vec![0.0; result_len];

        for (i, &a) in self.coefficients.iter().enumerate() {
            for (j, &b) in other.coefficients.iter().enumerate() {
                product[i + j] += a * b;
            }
        }

        Self::new(product)
    }

    /// Returns this polynomial scaled by `scalar`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_polynomial::Polynomial;
    ///
    /// let p = Polynomial::linear(1.0, 2.0);
    /// assert_eq!(p.scale(3.0).coefficients(), &[3.0, 6.0]);
    ///
    /// // Scaling by zero yields the zero polynomial.
    /// assert!(p.scale(0.0).is_zero());
    /// ```
    #[must_use]
    pub fn scale(&self, scalar: f64) -> Self {
        let scaled = self.coefficients.iter().map(|&c| c * scalar).collect();
        Self::new(scaled)
    }

    /// Returns `true` if this is the zero polynomial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_polynomial::Polynomial;
    ///
    /// assert!(Polynomial::zero().is_zero());
    /// assert!(!Polynomial::constant(1.0).is_zero());
    /// ```
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.coefficients.is_empty()
    }

    /// Removes trailing zero coefficients so that the internal representation
    /// stays canonical.
    fn normalize(mut coefficients: Vec<f64>) -> Vec<f64> {
        while coefficients.last().is_some_and(|&c| c == 0.0) {
            coefficients.pop();
        }
        coefficients
    }
}
