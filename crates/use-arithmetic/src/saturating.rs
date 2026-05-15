#![allow(clippy::module_name_repetitions)]

mod sealed {
    pub trait Sealed {}

    macro_rules! impl_sealed {
        ($($ty:ty),* $(,)?) => {
            $(impl Sealed for $ty {})*
        };
    }

    impl_sealed!(
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
    );
}

/// Primitive integer types supported by the saturating arithmetic wrappers.
pub trait SaturatingArithmetic: sealed::Sealed + Sized {
    /// Returns the saturating sum of `self` and `rhs`.
    #[must_use]
    fn saturating_add(self, rhs: Self) -> Self;

    /// Returns the saturating difference of `self` and `rhs`.
    #[must_use]
    fn saturating_sub(self, rhs: Self) -> Self;

    /// Returns the saturating product of `self` and `rhs`.
    #[must_use]
    fn saturating_mul(self, rhs: Self) -> Self;
}

macro_rules! impl_saturating_arithmetic {
    ($($ty:ty),* $(,)?) => {
        $(impl SaturatingArithmetic for $ty {
            fn saturating_add(self, rhs: Self) -> Self {
                <$ty>::saturating_add(self, rhs)
            }

            fn saturating_sub(self, rhs: Self) -> Self {
                <$ty>::saturating_sub(self, rhs)
            }

            fn saturating_mul(self, rhs: Self) -> Self {
                <$ty>::saturating_mul(self, rhs)
            }
        })*
    };
}

impl_saturating_arithmetic!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

/// Returns the saturating sum of `left` and `right`.
#[must_use]
pub fn saturating_add<T: SaturatingArithmetic>(left: T, right: T) -> T {
    <T as SaturatingArithmetic>::saturating_add(left, right)
}

/// Returns the saturating difference of `left` and `right`.
#[must_use]
pub fn saturating_sub<T: SaturatingArithmetic>(left: T, right: T) -> T {
    <T as SaturatingArithmetic>::saturating_sub(left, right)
}

/// Returns the saturating product of `left` and `right`.
#[must_use]
pub fn saturating_mul<T: SaturatingArithmetic>(left: T, right: T) -> T {
    <T as SaturatingArithmetic>::saturating_mul(left, right)
}

#[cfg(test)]
mod tests {
    use super::{saturating_add, saturating_mul, saturating_sub};

    #[test]
    fn saturates_unsigned_values() {
        assert_eq!(saturating_add(u8::MAX, 1), u8::MAX);
        assert_eq!(saturating_sub(0_u8, 1), 0);
        assert_eq!(saturating_mul(200_u8, 2), u8::MAX);
    }

    #[test]
    fn saturates_signed_values() {
        assert_eq!(saturating_add(i8::MAX, 1), i8::MAX);
        assert_eq!(saturating_sub(i8::MIN, 1), i8::MIN);
        assert_eq!(saturating_mul(i8::MAX, 2), i8::MAX);
    }
}
