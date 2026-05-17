#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small interval and bound primitives for `RustUse`.

#[doc(inline)]
pub use bound::Bound;
#[doc(inline)]
pub use interval::Interval;

/// Bound primitives for interval endpoints.
pub mod bound {
    /// A lower or upper bound for an interval endpoint.
    ///
    /// `Open` excludes the endpoint, `Closed` includes it, and `Unbounded`
    /// leaves the side unconstrained.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Bound<T> {
        /// An excluded endpoint.
        Open(T),
        /// An included endpoint.
        Closed(T),
        /// No endpoint on this side of the interval.
        Unbounded,
    }
}

/// Interval primitives and operations.
pub mod interval {
    use core::cmp::Ordering;

    use crate::bound::Bound;

    /// A mathematical interval described by a lower and upper bound.
    ///
    /// Intervals may be open, closed, half-open, or unbounded on either side.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_interval::{Bound, Interval};
    ///
    /// let closed = Interval::closed(1.0, 3.0);
    /// let half_open = Interval::new(Bound::Closed(0.0), Bound::Open(1.0));
    ///
    /// assert!(closed.contains(2.0));
    /// assert!(half_open.contains(0.5));
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Interval<T> {
        /// The lower endpoint rule.
        pub lower: Bound<T>,
        /// The upper endpoint rule.
        pub upper: Bound<T>,
    }

    impl<T> Interval<T> {
        /// Creates an interval from explicit lower and upper bounds.
        #[must_use]
        pub const fn new(lower: Bound<T>, upper: Bound<T>) -> Self {
            Self { lower, upper }
        }

        /// Creates an open interval `(lower, upper)`.
        #[must_use]
        pub const fn open(lower: T, upper: T) -> Self {
            Self::new(Bound::Open(lower), Bound::Open(upper))
        }

        /// Creates a closed interval `[lower, upper]`.
        #[must_use]
        pub const fn closed(lower: T, upper: T) -> Self {
            Self::new(Bound::Closed(lower), Bound::Closed(upper))
        }

        /// Creates a half-open interval `(lower, upper]`.
        #[must_use]
        pub const fn open_closed(lower: T, upper: T) -> Self {
            Self::new(Bound::Open(lower), Bound::Closed(upper))
        }

        /// Creates a half-open interval `[lower, upper)`.
        #[must_use]
        pub const fn closed_open(lower: T, upper: T) -> Self {
            Self::new(Bound::Closed(lower), Bound::Open(upper))
        }

        /// Creates the interval `(lower, +inf)`.
        #[must_use]
        pub const fn greater_than(lower: T) -> Self {
            Self::new(Bound::Open(lower), Bound::Unbounded)
        }

        /// Creates the interval `[lower, +inf)`.
        #[must_use]
        pub const fn greater_than_or_equal(lower: T) -> Self {
            Self::new(Bound::Closed(lower), Bound::Unbounded)
        }

        /// Creates the interval `(-inf, upper)`.
        #[must_use]
        pub const fn less_than(upper: T) -> Self {
            Self::new(Bound::Unbounded, Bound::Open(upper))
        }

        /// Creates the interval `(-inf, upper]`.
        #[must_use]
        pub const fn less_than_or_equal(upper: T) -> Self {
            Self::new(Bound::Unbounded, Bound::Closed(upper))
        }

        /// Creates the interval `(-inf, +inf)`.
        #[must_use]
        pub const fn unbounded() -> Self {
            Self::new(Bound::Unbounded, Bound::Unbounded)
        }
    }

    impl<T: PartialOrd + Copy> Interval<T> {
        /// Returns `true` when `value` lies inside the interval.
        #[must_use]
        pub fn contains(&self, value: T) -> bool {
            if self.is_empty() {
                return false;
            }

            lower_contains(&self.lower, &value) && upper_contains(&self.upper, &value)
        }

        /// Returns `true` when the interval contains no values.
        ///
        /// `(a, a)`, `(a, a]`, and `[a, a)` are empty. `[a, a]` is not empty.
        #[must_use]
        pub fn is_empty(&self) -> bool {
            match (bound_value(&self.lower), bound_value(&self.upper)) {
                (Some(lower), Some(upper)) => match lower.partial_cmp(upper) {
                    Some(Ordering::Less) => false,
                    Some(Ordering::Greater) | None => true,
                    Some(Ordering::Equal) => {
                        !matches!(self.lower, Bound::Closed(_))
                            || !matches!(self.upper, Bound::Closed(_))
                    },
                },
                _ => false,
            }
        }

        /// Returns `true` when the interval has both lower and upper bounds.
        #[must_use]
        pub const fn is_bounded(&self) -> bool {
            self.has_lower_bound() && self.has_upper_bound()
        }

        /// Returns `true` when the interval has a lower bound.
        #[must_use]
        pub const fn has_lower_bound(&self) -> bool {
            !matches!(self.lower, Bound::Unbounded)
        }

        /// Returns `true` when the interval has an upper bound.
        #[must_use]
        pub const fn has_upper_bound(&self) -> bool {
            !matches!(self.upper, Bound::Unbounded)
        }

        /// Returns `true` when the interval and `other` share at least one value.
        #[must_use]
        pub fn overlaps(&self, other: &Self) -> bool {
            self.intersection(other).is_some()
        }

        /// Returns the intersection with `other`, or `None` when it is empty.
        #[must_use]
        pub fn intersection(&self, other: &Self) -> Option<Self> {
            let interval = Self::new(
                max_lower_bound(self.lower, other.lower),
                min_upper_bound(self.upper, other.upper),
            );

            (!interval.is_empty()).then_some(interval)
        }
    }

    impl Interval<f64> {
        /// Returns the bounded interval length.
        ///
        /// Returns `None` for unbounded intervals. Empty bounded intervals have
        /// length `0.0`.
        #[must_use]
        pub fn length(&self) -> Option<f64> {
            match (self.lower, self.upper) {
                (
                    Bound::Open(lower) | Bound::Closed(lower),
                    Bound::Open(upper) | Bound::Closed(upper),
                ) => {
                    if self.is_empty() {
                        Some(0.0)
                    } else {
                        Some(upper - lower)
                    }
                },
                _ => None,
            }
        }
    }

    const fn bound_value<T>(bound: &Bound<T>) -> Option<&T> {
        match bound {
            Bound::Open(value) | Bound::Closed(value) => Some(value),
            Bound::Unbounded => None,
        }
    }

    fn lower_contains<T: PartialOrd>(bound: &Bound<T>, value: &T) -> bool {
        match bound {
            Bound::Open(lower) => value > lower,
            Bound::Closed(lower) => value >= lower,
            Bound::Unbounded => true,
        }
    }

    fn upper_contains<T: PartialOrd>(bound: &Bound<T>, value: &T) -> bool {
        match bound {
            Bound::Open(upper) => value < upper,
            Bound::Closed(upper) => value <= upper,
            Bound::Unbounded => true,
        }
    }

    fn max_lower_bound<T: PartialOrd + Copy>(left: Bound<T>, right: Bound<T>) -> Bound<T> {
        match (&left, &right) {
            (Bound::Unbounded, _) => right,
            (_, Bound::Unbounded) => left,
            (
                Bound::Open(left_value) | Bound::Closed(left_value),
                Bound::Open(right_value) | Bound::Closed(right_value),
            ) => match left_value.partial_cmp(right_value) {
                Some(Ordering::Less) => right,
                Some(Ordering::Greater) | None => left,
                Some(Ordering::Equal) => {
                    if matches!(left, Bound::Open(_)) || matches!(right, Bound::Open(_)) {
                        Bound::Open(*left_value)
                    } else {
                        Bound::Closed(*left_value)
                    }
                },
            },
        }
    }

    fn min_upper_bound<T: PartialOrd + Copy>(left: Bound<T>, right: Bound<T>) -> Bound<T> {
        match (&left, &right) {
            (Bound::Unbounded, _) => right,
            (_, Bound::Unbounded) => left,
            (
                Bound::Open(left_value) | Bound::Closed(left_value),
                Bound::Open(right_value) | Bound::Closed(right_value),
            ) => match left_value.partial_cmp(right_value) {
                Some(Ordering::Less) => left,
                Some(Ordering::Greater) | None => right,
                Some(Ordering::Equal) => {
                    if matches!(left, Bound::Open(_)) || matches!(right, Bound::Open(_)) {
                        Bound::Open(*left_value)
                    } else {
                        Bound::Closed(*left_value)
                    }
                },
            },
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Interval;
        use crate::Bound;

        #[test]
        fn open_intervals_exclude_endpoints() {
            let interval = Interval::open(1.0, 3.0);

            assert!(!interval.contains(1.0));
            assert!(interval.contains(2.0));
            assert!(!interval.contains(3.0));
        }

        #[test]
        fn closed_intervals_include_endpoints() {
            let interval = Interval::closed(1.0, 3.0);

            assert!(interval.contains(1.0));
            assert!(interval.contains(3.0));
            assert!(!interval.contains(4.0));
        }

        #[test]
        fn half_open_intervals_respect_each_endpoint() {
            let left_closed = Interval::closed_open(1.0, 3.0);
            let right_closed = Interval::open_closed(1.0, 3.0);

            assert!(left_closed.contains(1.0));
            assert!(!left_closed.contains(3.0));
            assert!(!right_closed.contains(1.0));
            assert!(right_closed.contains(3.0));
        }

        #[test]
        fn unbounded_intervals_work_correctly() {
            let above = Interval::greater_than_or_equal(2.0);
            let below = Interval::less_than(5.0);
            let everywhere = Interval::<f64>::unbounded();

            assert!(above.contains(2.0));
            assert!(above.contains(10.0));
            assert!(!above.contains(1.5));
            assert!(below.contains(-100.0));
            assert!(!below.contains(5.0));
            assert!(everywhere.contains(0.0));
            assert!(everywhere.overlaps(&above));
            assert!(above.has_lower_bound());
            assert!(!above.has_upper_bound());
            assert!(!everywhere.is_bounded());
        }

        #[test]
        fn same_endpoint_rules_define_emptiness() {
            assert!(Interval::open(2.0, 2.0).is_empty());
            assert!(Interval::open_closed(2.0, 2.0).is_empty());
            assert!(Interval::closed_open(2.0, 2.0).is_empty());
            assert!(!Interval::closed(2.0, 2.0).is_empty());
        }

        #[test]
        fn lower_greater_than_upper_is_empty() {
            let interval = Interval::closed(4.0, 2.0);

            assert!(interval.is_empty());
            assert!(!interval.contains(3.0));
            assert_eq!(interval.length(), Some(0.0));
        }

        #[test]
        fn overlaps_require_a_shared_included_point() {
            let closed = Interval::closed(1.0, 3.0);
            let open = Interval::open(3.0, 5.0);
            let also_closed = Interval::closed(3.0, 5.0);

            assert!(!closed.overlaps(&open));
            assert!(closed.overlaps(&also_closed));
        }

        #[test]
        fn intersection_returns_none_when_empty() {
            let left = Interval::closed(1.0, 3.0);
            let right = Interval::open(3.0, 5.0);

            assert_eq!(left.intersection(&right), None);
        }

        #[test]
        fn intersection_keeps_shared_included_endpoint() {
            let left = Interval::closed(1.0, 3.0);
            let right = Interval::closed(3.0, 5.0);

            assert_eq!(left.intersection(&right), Some(Interval::closed(3.0, 3.0)));
        }

        #[test]
        fn intersection_of_unbounded_intervals_stays_precise() {
            let left = Interval::greater_than_or_equal(1.0);
            let right = Interval::less_than_or_equal(4.0);

            assert_eq!(
                left.intersection(&right),
                Some(Interval::new(Bound::Closed(1.0), Bound::Closed(4.0)))
            );
        }

        #[test]
        fn length_handles_bounded_and_unbounded_intervals() {
            assert_eq!(Interval::closed(2.0, 2.0).length(), Some(0.0));
            assert_eq!(Interval::open(1.0, 4.5).length(), Some(3.5));
            assert_eq!(Interval::greater_than(1.0).length(), None);
            assert_eq!(Interval::less_than_or_equal(4.0).length(), None);
        }
    }
}
