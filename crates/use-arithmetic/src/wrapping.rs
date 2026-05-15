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

/// Primitive integer types supported by the wrapping arithmetic wrappers.
pub trait WrappingArithmetic: sealed::Sealed + Sized {
    /// Returns the wrapping sum of `self` and `rhs`.
    #[must_use]
    fn wrapping_add(self, rhs: Self) -> Self;

    /// Returns the wrapping difference of `self` and `rhs`.
    #[must_use]
    fn wrapping_sub(self, rhs: Self) -> Self;

    /// Returns the wrapping product of `self` and `rhs`.
    #[must_use]
    fn wrapping_mul(self, rhs: Self) -> Self;
}

macro_rules! impl_wrapping_arithmetic {
    ($($ty:ty),* $(,)?) => {
        $(impl WrappingArithmetic for $ty {
            fn wrapping_add(self, rhs: Self) -> Self {
                <$ty>::wrapping_add(self, rhs)
            }

            fn wrapping_sub(self, rhs: Self) -> Self {
                <$ty>::wrapping_sub(self, rhs)
            }

            fn wrapping_mul(self, rhs: Self) -> Self {
                <$ty>::wrapping_mul(self, rhs)
            }
        })*
    };
}

impl_wrapping_arithmetic!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

/// Returns the wrapping sum of `left` and `right`.
#[must_use]
pub fn wrapping_add<T: WrappingArithmetic>(left: T, right: T) -> T {
    <T as WrappingArithmetic>::wrapping_add(left, right)
}

/// Returns the wrapping difference of `left` and `right`.
#[must_use]
pub fn wrapping_sub<T: WrappingArithmetic>(left: T, right: T) -> T {
    <T as WrappingArithmetic>::wrapping_sub(left, right)
}

/// Returns the wrapping product of `left` and `right`.
#[must_use]
pub fn wrapping_mul<T: WrappingArithmetic>(left: T, right: T) -> T {
    <T as WrappingArithmetic>::wrapping_mul(left, right)
}

#[cfg(test)]
mod tests {
    use super::{wrapping_add, wrapping_mul, wrapping_sub};

    #[test]
    fn wraps_unsigned_values() {
        assert_eq!(wrapping_add(u8::MAX, 1), 0);
        assert_eq!(wrapping_sub(0_u8, 1), u8::MAX);
        assert_eq!(wrapping_mul(200_u8, 2), 144);
    }

    #[test]
    fn wraps_signed_values() {
        assert_eq!(wrapping_add(i8::MAX, 1), i8::MIN);
        assert_eq!(wrapping_sub(i8::MIN, 1), i8::MAX);
        assert_eq!(wrapping_mul(100_i8, 2), -56);
    }
}
